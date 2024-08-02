/*
 * Copyright 2020 fsyncd, Berlin, Germany.
 * Additional material, copyright of the containerd authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[macro_use]
extern crate derive_builder;

/// Spec is the base configuration for the container.
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct Spec {
    /// Version of the Open Container Initiative Runtime Specification with which the bundle complies.
    #[serde(rename = "ociVersion")]
    version: String,
    /// Process configures the container process.
    #[serde(skip_serializing_if = "Option::is_none")]
    process: Option<Process>,
    /// Root configures the container's root filesystem.
    #[serde(skip_serializing_if = "Option::is_none")]
    root: Option<Root>,
    /// Hostname configures the container's hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<String>,
    /// Mounts configures additional mounts (on top of Root).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    mounts: Vec<Mount>,
    /// Hooks configures callbacks for container lifecycle events.
    #[serde(skip_serializing_if = "Option::is_none")]
    hooks: Option<Hooks>,
    /// Annotations contains arbitrary metadata for the container.
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    annotations: HashMap<String, String>,
    /// Linux is platform-specific configuration for Linux based containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    linux: Option<Linux>,
}

/// Process contains information to start a specific application inside the container.
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct Process {
    /// Terminal creates an interactive terminal for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    terminal: Option<bool>,
    /// ConsoleSize specifies the size of the console.
    #[serde(skip_serializing_if = "Option::is_none", rename = "consoleSize")]
    console_size: Option<ConsoleSizeBox>,
    /// User specifies user information for the process.
    user: User,
    /// Args specifies the binary and arguments for the application to execute.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
    /// Env populates the process environment for the process.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    env: Vec<String>,
    /// Cwd is the current working directory for the process and must be
    /// relative to the container's root.
    cwd: String,
    /// Capabilities are Linux capabilities that are kept for the process.
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<LinuxCapabilities>,
    /// Rlimits specifies rlimit options to apply to the process.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    rlimits: Vec<POSIXRlimit>,
    /// NoNewPrivileges controls whether additional privileges could be gained by processes in the container.
    #[serde(skip_serializing_if = "Option::is_none", rename = "noNewPrivileges")]
    no_new_privileges: Option<bool>,
    /// ApparmorProfile specifies the apparmor profile for the container.
    #[serde(skip_serializing_if = "Option::is_none", rename = "apparmorProfile")]
    app_armor_profile: Option<String>,
    /// Specify an oom_score_adj for the container.
    #[serde(skip_serializing_if = "Option::is_none", rename = "oomScoreAdj")]
    oom_score_adj: Option<i32>,
    /// SelinuxLabel specifies the selinux context that the container process is run as.
    #[serde(skip_serializing_if = "Option::is_none", rename = "selinuxLabel")]
    selinux_label: Option<String>,
}

/// LinuxCapabilities specifies the whitelist of capabilities that are kept for a process.
/// http://man7.org/linux/man-pages/man7/capabilities.7.html
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxCapabilities {
    /// Bounding is the set of capabilities checked by the kernel.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    bounding: Vec<String>,
    /// Effective is the set of capabilities checked by the kernel.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    effective: Vec<String>,
    /// Inheritable is the capabilities preserved across execve.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    inheritable: Vec<String>,
    /// Permitted is the limiting superset for effective capabilities.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    permitted: Vec<String>,
    /// Ambient is the ambient set of capabilities that are kept.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    ambient: Vec<String>,
}

/// ConsoleSizeBox specifies dimensions of a rectangle. Used for specifying the size of a console.
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct ConsoleSizeBox {
    /// Height is the vertical dimension of a box.
    height: u32,
    /// Width is the horizontal dimension of a box.
    width: u32,
}

/// User specifies specific user (and group) information for the container process.
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct User {
    /// UID is the user id.
    uid: u32,
    /// GID is the group id.
    gid: u32,
    /// Umask is the umask for the init process.
    #[serde(skip_serializing_if = "Option::is_none")]
    umask: Option<u32>,
    /// AdditionalGids are additional group ids set for the container's process.
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "additionalGids", default)]
    additional_gids: Vec<u32>,
}

/// Root contains information about the container's root filesystem on the host.
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct Root {
    /// Path is the absolute path to the container's root filesystem.
    path: String,
    /// Readonly makes the root filesystem for the container readonly before the process is executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    readonly: Option<bool>,
}

/// Mount specifies a mount for a container.
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct Mount {
    /// Destination is the absolute path where the mount will be placed in the container.
    destination: String,
    /// Type specifies the mount kind.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    mount_type: Option<String>,
    /// Source specifies the source path of the mount.
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    /// Options are fstab style mount options.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    options: Vec<String>,
}

/// Hook specifies a command that is run at a particular event in the lifecycle of a container
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct Hook {
    path: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    env: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<i32>,
}

/// Hooks specifies a command that is run in the container at a particular event in the lifecycle of a container
/// Hooks for container setup and teardown
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct Hooks {
    /// Prestart is Deprecated. Prestart is a list of hooks to be run before the container process is executed.
    /// It is called in the Runtime Namespace
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    prestart: Vec<Hook>,
    /// CreateRuntime is a list of hooks to be run after the container has been created but before pivot_root or any equivalent operation has been called
    /// It is called in the Runtime Namespace
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "createRuntime", default)]
    create_runtime: Vec<Hook>,
    /// CreateContainer is a list of hooks to be run after the container has been created but before pivot_root or any equivalent operation has been called
    /// It is called in the Container Namespace
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "createContainer", default)]
    create_container: Vec<Hook>,
    /// StartContainer is a list of hooks to be run after the start operation is called but before the container process is started
    /// It is called in the Container Namespace
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "startContainer", default)]
    start_container: Vec<Hook>,
    /// Poststart is a list of hooks to be run after the container process is started.
    /// It is called in the Runtime Namespace
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    poststart: Vec<String>,
    /// Poststop is a list of hooks to be run after the container process exits.
    /// It is called in the Runtime Namespace
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    poststop: Vec<String>,
}

/// Linux contains platform-specific configuration for Linux based containers.
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct Linux {
    /// UIDMapping specifies user mappings for supporting user namespaces.
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "uidMappings", default)]
    uid_mappings: Vec<LinuxIDMapping>,
    /// GIDMapping specifies group mappings for supporting user namespaces.
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "gidMappings", default)]
    gid_mappings: Vec<LinuxIDMapping>,
    /// Sysctl are a set of key value pairs that are set for the container on start
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    sysctl: HashMap<String, String>,
    /// Resources contain cgroup information for handling resource constraints
    /// for the container
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<LinuxResources>,
    /// CgroupsPath specifies the path to cgroups that are created and/or joined by the container.
    /// The path is expected to be relative to the cgroups mountpoint.
    /// If resources are specified, the cgroups at CgroupsPath will be updated based on resources.
    #[serde(skip_serializing_if = "Option::is_none", rename = "cgroupsPath")]
    cgroups_path: Option<String>,
    /// Namespaces contains the namespaces that are created and/or joined by the container
    #[serde(skip_serializing_if = "Vec::is_empty")]
    namespaces: Vec<LinuxNamespace>,
    /// Devices are a list of device nodes that are created for the container
    #[serde(skip_serializing_if = "Vec::is_empty")]
    devices: Vec<LinuxDevice>,
    /// Seccomp specifies the seccomp security settings for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    seccomp: Option<LinuxSeccomp>,
    /// RootfsPropagation is the rootfs mount propagation mode for the container.
    #[serde(skip_serializing_if = "Option::is_none", rename = "rootfsPropagation")]
    rootfs_propagation: Option<String>,
    /// MaskedPaths masks over the provided paths inside the container.
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "maskedPaths")]
    masked_paths: Vec<String>,
    /// ReadonlyPaths sets the provided paths as RO inside the container.
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "readonlyPaths")]
    readonly_paths: Vec<String>,
    /// MountLabel specifies the selinux context for the mounts in the container.
    #[serde(skip_serializing_if = "Option::is_none", rename = "mountLabel")]
    mount_label: Option<String>,
    /// IntelRdt contains Intel Resource Director Technology (RDT) information for
    /// handling resource constraints (e.g., L3 cache, memory bandwidth) for the container
    #[serde(skip_serializing_if = "Option::is_none", rename = "intelRdt")]
    intel_rdt: Option<LinuxIntelRdt>,
    /// Personality contains configuration for the Linux personality syscall
    #[serde(skip_serializing_if = "Option::is_none")]
    personality: Option<LinuxPersonality>,
}

/// LinuxNamespace is the configuration for a Linux namespace
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxNamespace {
    /// Type is the type of namespace
    #[serde(rename = "type")]
    namespace_type: String,
    /// Path is a path to an existing namespace persisted on disk that can be joined
    /// and is of the same type
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
}

/// LinuxIDMapping specifies UID/GID mappings
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxIDMapping {
    /// ContainerID is the starting UID/GID in the container
    #[serde(rename = "containerID")]
    container_id: u32,
    /// HostID is the starting UID/GID on the host to be mapped to 'ContainerID'
    #[serde(rename = "hostID")]
    host_id: u32,
    /// Size is the number of IDs to be mapped
    size: u32,
}

/// POSIXRlimit type and restrictions
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct POSIXRlimit {
    /// Type of the rlimit to set
    #[serde(rename = "type")]
    rlimit_type: String,
    /// Hard is the hard limit for the specified type
    hard: u64,
    /// Soft is the soft limit for the specified type
    soft: u64,
}

// LinuxHugepageLimit structure corresponds to limiting kernel hugepages
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxHugepageLimit {
    /// Pagesize is the hugepage size
    /// Format: "<size><unit-prefix>B' (e.g. 64KB, 2MB, 1GB, etc.)
    #[serde(rename = "pageSize")]
    page_size: String,
    /// Limit is the limit of "hugepagesize" hugetlb usage
    limit: u64,
}

/// LinuxInterfacePriority for network interfaces
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxInterfacePriority {
    /// Name is the name of the network interface
    name: String,
    /// Priority for the interface
    priority: u32,
}

/// LinuxWeightDevice struct holds a `major:minor weight` pair for weightDevice
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxWeightDevice {
    /// Major is the device's major number.
    major: i64,
    /// Minor is the device's minor number.
    minor: i64,
    /// Weight is the bandwidth rate for the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<u16>,
    /// LeafWeight is the bandwidth rate for the device while competing with the cgroup's child cgroups, CFQ scheduler only
    #[serde(skip_serializing_if = "Option::is_none", rename = "leafWeight")]
    leaf_weight: Option<u16>,
}

/// LinuxThrottleDevice struct holds a `major:minor rate_per_second` pai
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxThrottleDevice {
    /// Major is the device's major number.
    major: i64,
    /// Minor is the device's minor number.
    minor: i64,
    /// Rate is the IO rate limit per cgroup per device
    rate: u64,
}

/// LinuxBlockIO for Linux cgroup 'blkio' resource management
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxBlockIO {
    /// Specifies per cgroup weight
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<u16>,
    /// Specifies tasks' weight in the given cgroup while competing with the cgroup's child cgroups, CFQ scheduler only
    #[serde(skip_serializing_if = "Option::is_none", rename = "leafWeight")]
    leaf_weight: Option<u16>,
    /// Weight per cgroup per device, can override BlkioWeight
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "weightDevice")]
    weight_device: Vec<LinuxWeightDevice>,
    /// IO read rate limit per cgroup per device, bytes per second
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        rename = "throttleReadBpsDevice"
    )]
    throttle_read_bps_device: Vec<LinuxThrottleDevice>,
    /// IO write rate limit per cgroup per device, bytes per second
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        rename = "throttleWriteBpsDevice"
    )]
    throttle_write_bps_device: Vec<LinuxThrottleDevice>,
    /// IO read rate limit per cgroup per device, IO per second
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        rename = "throttleReadIOPSDevice"
    )]
    throttle_read_iops_device: Vec<LinuxThrottleDevice>,
    /// IO write rate limit per cgroup per device, IO per second
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        rename = "throttleWriteIOPSDevice"
    )]
    throttle_write_iops_device: Vec<LinuxThrottleDevice>,
}

/// LinuxMemory for Linux cgroup 'memory' resource management
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxMemory {
    /// Memory limit (in bytes).
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    /// Memory reservation or soft_limit (in bytes).
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation: Option<i64>,
    /// Total memory limit (memory + swap).
    #[serde(skip_serializing_if = "Option::is_none")]
    swap: Option<i64>,
    /// Kernel memory limit (in bytes).
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel: Option<i64>,
    /// Kernel memory limit for tcp (in bytes)
    #[serde(skip_serializing_if = "Option::is_none", rename = "kernelTCP")]
    kernel_tcp: Option<i64>,
    /// How aggressive the kernel will swap memory pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    swappiness: Option<i64>,
    /// DisableOOMKiller disables the OOM killer for out of memory conditions
    #[serde(skip_serializing_if = "Option::is_none", rename = "disableOOMKiller")]
    disable_oom_killer: Option<bool>,
    /// Enables hierarchical memory accounting
    #[serde(skip_serializing_if = "Option::is_none", rename = "useHierarchy")]
    use_hierarchy: Option<bool>,
}

/// LinuxCPU for Linux cgroup 'cpu' resource management
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxCPU {
    /// CPU shares (relative weight (ratio) vs. other cgroups with cpu shares).
    #[serde(skip_serializing_if = "Option::is_none")]
    shares: Option<u64>,
    /// CPU hardcap limit (in usecs). Allowed cpu time in a given period.
    #[serde(skip_serializing_if = "Option::is_none")]
    quota: Option<i64>,
    /// CPU period to be used for hardcapping (in usecs).
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<u64>,
    /// How much time realtime scheduling may use (in usecs).
    #[serde(skip_serializing_if = "Option::is_none", rename = "realtimeRuntime")]
    realtime_runtime: Option<i64>,
    /// CPU period to be used for realtime scheduling (in usecs).
    #[serde(skip_serializing_if = "Option::is_none", rename = "realtimePeriod")]
    realtime_period: Option<u64>,
    /// CPUs to use within the cpuset. Default is to use any CPU available.
    #[serde(skip_serializing_if = "Option::is_none")]
    cpus: Option<String>,
    /// List of memory nodes in the cpuset. Default is to use any available memory node.
    #[serde(skip_serializing_if = "Option::is_none")]
    mems: Option<String>,
}

/// LinuxPids for Linux cgroup 'pids' resource management (Linux 4.3)
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxPids {
    /// Maximum number of PIDs. Default is "no limit".
    limit: i64,
}

/// LinuxNetwork identification and priority configuration
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxNetwork {
    /// Set class identifier for container's network packets
    #[serde(skip_serializing_if = "Option::is_none", rename = "classID")]
    class_id: Option<u32>,
    /// Set priority of network traffic for container
    #[serde(skip_serializing_if = "Vec::is_empty")]
    priorities: Vec<LinuxInterfacePriority>,
}

/// LinuxRdma for Linux cgroup 'rdma' resource management (Linux 4.11)
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxRdma {
    /// Maximum number of HCA handles that can be opened. Default is "no limit".
    #[serde(skip_serializing_if = "Option::is_none", rename = "hcaHandles")]
    hca_handles: Option<u32>,
    /// Maximum number of HCA objects that can be created. Default is "no limit".
    #[serde(skip_serializing_if = "Option::is_none", rename = "hcaObjects")]
    hca_objects: Option<u32>,
}

/// LinuxResources has container runtime resource constraints
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxResources {
    /// Devices configures the device whitelist.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    devices: Vec<LinuxDeviceCgroup>,
    /// Memory restriction configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<LinuxMemory>,
    /// CPU resource restriction configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<LinuxCPU>,
    /// Task resource restriction configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pids: Option<LinuxPids>,
    /// BlockIO restriction configuration
    #[serde(skip_serializing_if = "Option::is_none", rename = "blockIO")]
    block_io: Option<LinuxBlockIO>,
    /// Hugetlb limit (in bytes)
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "hugepageLimits", default)]
    hugepage_limits: Vec<LinuxHugepageLimit>,
    /// Network restriction configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<LinuxNetwork>,
    /// Rdma resource restriction configuration.
    /// Limits are a set of key value pairs that define RDMA resource limits,
    /// where the key is device name and value is resource limits.
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    rdma: HashMap<String, LinuxRdma>,
}

/// LinuxDevice represents the mknod information for a Linux special device file
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxDevice {
    /// Path to the device.
    path: String,
    /// Device type, block, char, etc.
    #[serde(rename = "type")]
    device_type: String,
    /// Major is the device's major number.
    major: i64,
    /// Minor is the device's minor number.
    minor: i64,
    /// FileMode permission bits for the device.
    #[serde(skip_serializing_if = "Option::is_none", rename = "fileMode")]
    file_mode: Option<u32>,
    /// UID of the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<u32>,
    /// Gid of the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    gid: Option<u32>,
}

/// LinuxDeviceCgroup represents a device rule for the whitelist controller
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxDeviceCgroup {
    /// Allow or deny
    allow: bool,
    /// Device type, block, char, etc.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    device_type: Option<String>,
    /// Major is the device's major number.
    #[serde(skip_serializing_if = "Option::is_none")]
    major: Option<i64>,
    /// Minor is the device's minor number.
    #[serde(skip_serializing_if = "Option::is_none")]
    minor: Option<i64>,
    /// Cgroup access permissions format, rwm.
    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<String>,
}

/// LinuxPersonality represents the Linux personality syscall input
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxPersonality {
    /// Domain for the personality
    domain: String,
    /// Additional flags
    #[serde(skip_serializing_if = "Vec::is_empty")]
    flags: Vec<String>,
}

/// LinuxSeccomp represents syscall restrictions
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxSeccomp {
    #[serde(rename = "defaultAction")]
    default_action: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    architectures: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    flags: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    syscalls: Vec<LinuxSyscall>,
}

/// LinuxSeccompArg used for matching specific syscall arguments in Seccomp
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxSeccompArg {
    index: u64,
    value: u64,
    #[serde(skip_serializing_if = "Option::is_none", rename = "valueTwo")]
    value_two: Option<u64>,
    op: String,
}

/// LinuxSyscall is used to match a syscall in Seccomp
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxSyscall {
    names: Vec<String>,
    action: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
}

/// LinuxIntelRdt has container runtime resource constraints for Intel RDT
/// CAT and MBA features which introduced in Linux 4.10 and 4.12 kernel
#[derive(Default, Clone, Builder, Debug, Serialize, Deserialize)]
#[builder(default, setter(into))]
pub struct LinuxIntelRdt {
    /// The identity for RDT Class of Service
    #[serde(skip_serializing_if = "Option::is_none", rename = "closID")]
    clos_id: Option<String>,
    /// The schema for L3 cache id and capacity bitmask (CBM)
    /// Format: "L3:<cache_id0>=<cbm0>;<cache_id1>=<cbm1>;..."
    #[serde(skip_serializing_if = "Option::is_none", rename = "l3CacheSchema")]
    l3_cache_schema: Option<String>,
    /// The schema of memory bandwidth per L3 cache id
    /// Format: "MB:<cache_id0>=bandwidth0;<cache_id1>=bandwidth1;..."
    /// The unit of memory bandwidth is specified in "percentages" by
    /// Default, Clone, and in "MBps" if MBA Software Controller is enabled.
    #[serde(skip_serializing_if = "Option::is_none", rename = "memBwSchema")]
    mem_bw_schema: Option<String>,
}
