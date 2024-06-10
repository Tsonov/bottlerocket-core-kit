use crate::{BootSettings, ContainerRuntimeSettings, KubernetesSettings, OciDefaults};

use model_derive::model;

// Note: we have to use 'rename' here because the top-level Settings structure is the only one
// that uses its name in serialization; internal structures use the field name that points to it
#[model(rename = "settings", impl_default = true)]
struct Settings {
    motd: settings_extension_motd::MotdV1,
    kubernetes: KubernetesSettings,
    updates: settings_extension_updates::UpdatesSettingsV1,
    host_containers: settings_extension_host_containers::HostContainersSettingsV1,
    bootstrap_containers: settings_extension_bootstrap_container::BootstrapContainerSettingsV1,
    ntp: settings_extension_ntp::NtpSettingsV1,
    network: settings_extension_network::NetworkSettingsV1,
    kernel: settings_extension_kernel::KernelSettingsV1,
    boot: BootSettings,
    aws: settings_extension_aws::AwsSettingsV1,
    metrics: settings_extension_metrics::MetricsSettingsV1,
    pki: settings_extension_pki::PkiSettingsV1,
    container_registry: settings_extension_container_registry::RegistrySettingsV1,
    oci_defaults: OciDefaults,
    oci_hooks: settings_extension_oci_hooks::OciHooksSettingsV1,
    cloudformation: settings_extension_cloudformation::CloudFormationSettingsV1,
    dns: settings_extension_dns::DnsSettingsV1,
    container_runtime: ContainerRuntimeSettings,
    autoscaling: settings_extension_autoscaling::AutoScalingSettingsV1,
}
