use crate::aws::sdk_config;
use crate::PROVIDER;
use aws_sdk_eks::types::KubernetesNetworkConfigResponse;
use aws_smithy_experimental::hyper_1_0::HyperClientBuilder;
use snafu::{OptionExt, ResultExt, Snafu};
use std::time::Duration;

// Limit the timeout for the EKS describe cluster API call to 5 minutes
const EKS_DESCRIBE_CLUSTER_TIMEOUT: Duration = Duration::from_secs(300);

pub(crate) type ClusterNetworkConfig = KubernetesNetworkConfigResponse;

#[derive(Debug, Snafu)]
pub(super) enum Error {
    #[snafu(display("Error describing cluster: {}", source))]
    DescribeCluster {
        source: aws_sdk_eks::error::SdkError<
            aws_sdk_eks::operation::describe_cluster::DescribeClusterError,
        >,
    },

    #[snafu(display("Timed-out waiting for EKS Describe Cluster API response: {}", source))]
    DescribeClusterTimeout { source: tokio::time::error::Elapsed },

    #[snafu(display("Missing field '{}' in EKS response", field))]
    Missing { field: &'static str },
}

type Result<T> = std::result::Result<T, Error>;

/// Returns the cluster's [kubernetesNetworkConfig] by calling the EKS API.
/// (https://docs.aws.amazon.com/eks/latest/APIReference/API_KubernetesNetworkConfigResponse.html)
pub(super) async fn get_cluster_network_config<H, N>(
    region: &str,
    cluster: &str,
    https_proxy: Option<H>,
    no_proxy: Option<&[N]>,
) -> Result<ClusterNetworkConfig>
where
    H: AsRef<str>,
    N: AsRef<str>,
{
    println!("get_cluster_network_config::start");
    let config = sdk_config(region).await;
    println!("get_cluster_network_config::got sdk config {:?}", config);

    let client = build_client(https_proxy, no_proxy, config)?;
    println!("get_cluster_network_config::client created");

    println!("get_cluster_network_config::calling EKS api");
    tokio::time::timeout(
        EKS_DESCRIBE_CLUSTER_TIMEOUT,
        client.describe_cluster().name(cluster.to_owned()).send(),
    )
    .await
    .inspect_err(|err| {
        println!("get_cluster_network_config::error while describing cluster {}", err);
    })
    .context(DescribeClusterTimeoutSnafu)?
    .context(DescribeClusterSnafu)?
    .cluster
    .context(MissingSnafu { field: "cluster" })?
    .kubernetes_network_config
    .inspect(|config| {
        println!("get_cluster_network_config::got k8s network config {:?}", config);
    })
    .context(MissingSnafu {
        field: "kubernetes_network_config",
    })
}

fn build_client<H, N>(
    https_proxy: Option<H>,
    no_proxy: Option<&[N]>,
    config: aws_config::SdkConfig,
) -> Result<aws_sdk_eks::Client>
where
    H: AsRef<str>,
    N: AsRef<str>,
{
    println!("build_client::start");
    let no_proxy_raw = match no_proxy {
        Some(v) => v,
        None => &[] as &[N],
    };

    let http_client = if let Some(https_proxy) = https_proxy {
        let https_proxy = https_proxy.as_ref().to_string();

        println!("build_client::http_client with proxy vals https_proxy {}, no_proxy {:?}", https_proxy, no_proxy_raw);
        HyperClientBuilder::new()
            .crypto_mode(PROVIDER)
            .build_with_proxy(https_proxy, no_proxy)
    } else {
        println!("build_client::http_client with no proxy");
        HyperClientBuilder::new()
            .crypto_mode(PROVIDER)
            .build_https()
    };
    println!("build_client::aws_config {:?}", config);
    let eks_config = aws_sdk_eks::config::Builder::from(&config)
        .http_client(http_client)
        .build();

    println!("build_client::eks_config {:?}", eks_config);

    println!("build_client::end");
    Ok(aws_sdk_eks::Client::from_conf(eks_config))
}
