//! This module corresponds to `mach/task.defs`.

use core::ffi::c_int;

use kern_return::kern_return_t;
use libc::{
    boolean_t, ledger_array_t, mach_vm_address_t, mach_vm_size_t, natural_t, task_inspect_t,
    vm_address_t,
};
use mach_types::{task_name_t, task_t, thread_act_array_t};
use message::mach_msg_type_number_t;
use port::{mach_port_array_t, mach_port_t};
use task_info::{task_flavor_t, task_info_t};
use vm_types::integer_t;

use crate::{
    dyld_kernel::{
        dyld_kernel_image_info_array_t, dyld_kernel_image_info_t, dyld_kernel_process_info_t,
    },
    exception_types::{
        exception_behavior_array_t, exception_behavior_t, exception_flavor_array_t,
        exception_mask_array_t, exception_mask_t,
    },
    mach_types::{
        exception_handler_array_t, lock_set_t, processor_set_name_t, processor_set_t, semaphore_t,
        task_policy_flavor_t, task_policy_set_t, task_policy_t, task_suspension_token_t,
        thread_act_t,
    },
    port::mach_port_name_t,
    task_info::task_purgable_info_t,
    thread_status::{thread_state_flavor_t, thread_state_t},
    vm_types::mach_vm_offset_t,
};

pub type task_special_port_t = ::libc::c_int;

pub type policy_t = ::libc::c_int;
pub type policy_base_t = *mut integer_t;
pub type policy_info_t = *mut integer_t;
pub type policy_limit_t = *mut integer_t;

pub type kcdata_object_t = mach_port_t;
pub type task_read_t = mach_port_t;
pub type emulation_vector_t = *mut mach_vm_offset_t;
pub type ipc_voucher_t = mach_port_t;
pub type task_inspect_flavor_t = natural_t;
pub type task_inspect_info_t = *mut integer_t;
pub type task_exc_guard_behavior_t = u32;
pub type task_corpse_forking_behavior_t = u32;
pub type task_id_token_t = mach_port_t;
pub type mach_voucher_selector_t = u32;

// pub type task_zone_info_array_t = *mut task_zone_info_t;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct exception_handler_info_value {
    pub one: natural_t,
    pub two: natural_t,
}
pub type exception_handler_info_t = exception_handler_info_value;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mach_port_qos_value {
    pub one: natural_t,
    pub two: natural_t,
}

pub type mach_port_qos_t = mach_port_qos_value;
pub type exception_handler_info_array_t = *mut exception_handler_info_t;

pub const TASK_KERNEL_PORT: task_special_port_t = 1;
pub const TASK_HOST_PORT: task_special_port_t = 2;
pub const TASK_NAME_PORT: task_special_port_t = 3;
pub const TASK_BOOTSTRAP_PORT: task_special_port_t = 4;

extern "C" {
    pub fn task_create(
        target_task: task_t,
        ledgers: ledger_array_t,
        ledgersCnt: mach_msg_type_number_t,
        inherit_memory: boolean_t,
        child_task: *mut task_t,
    ) -> kern_return_t;
    pub fn task_terminate(target_task: task_t) -> kern_return_t;
    pub fn task_resume(target_task: task_t) -> kern_return_t;
    pub fn task_suspend(target_task: task_t) -> kern_return_t;
    pub fn task_get_special_port(
        task: task_t,
        which_port: task_special_port_t,
        special_port: *mut mach_port_t,
    ) -> kern_return_t;
    pub fn task_set_special_port(
        task: task_t,
        which_port: task_special_port_t,
        special_port: mach_port_t,
    ) -> kern_return_t;
    pub fn task_threads(
        target_task: task_t,
        act_list: *mut thread_act_array_t,
        act_list_cnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_info(
        target_task: task_name_t,
        flavor: task_flavor_t,
        task_info_out: task_info_t,
        task_info_outCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_set_info(
        target_task: task_t,
        flavor: task_flavor_t,
        task_info_in: task_info_t,
        task_info_inCnt: mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn mach_ports_register(
        target_task: task_t,
        init_port_set: mach_port_array_t,
        init_port_setCnt: mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn mach_ports_lookup(
        target_task: task_t,
        init_port_set: *mut mach_port_array_t,
        init_port_setCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn thread_create(parent_task: task_t, child_act: *mut thread_act_t) -> kern_return_t;
    pub fn thread_create_running(
        parent_task: task_t,
        flavor: thread_state_flavor_t,
        new_state: thread_state_t,
        new_stateCnt: mach_msg_type_number_t,
        child_act: *mut thread_act_t,
    ) -> kern_return_t;
    pub fn task_set_exception_ports(
        task: task_t,
        exception_mask: exception_mask_t,
        new_port: mach_port_t,
        behavior: exception_behavior_t,
        new_flavor: thread_state_flavor_t,
    ) -> kern_return_t;
    pub fn task_get_exception_ports(
        task: task_t,
        exception_mask: exception_mask_t,
        masks: exception_mask_array_t,
        masksCnt: *mut mach_msg_type_number_t,
        old_handlers: exception_handler_array_t,
        old_behaviors: exception_behavior_array_t,
        old_flavors: exception_flavor_array_t,
    ) -> kern_return_t;
    pub fn task_swap_exception_ports(
        task: task_t,
        exception_mask: exception_mask_t,
        new_port: mach_port_t,
        behavior: exception_behavior_t,
        new_flavor: thread_state_flavor_t,
        masks: exception_mask_array_t,
        masksCnt: *mut mach_msg_type_number_t,
        old_handlers: exception_handler_array_t,
        old_behaviors: exception_behavior_array_t,
        old_flavors: exception_flavor_array_t,
    ) -> kern_return_t;
    pub fn lock_set_create(
        task: task_t,
        new_lock_set: *mut lock_set_t,
        n_ulocks: c_int,
        policy: c_int,
    ) -> kern_return_t;
    pub fn lock_set_destroy(task: task_t, lock_set: lock_set_t) -> kern_return_t;
    pub fn semaphore_create(
        task: task_t,
        semaphore: *mut semaphore_t,
        policy: c_int,
        value: c_int,
    ) -> kern_return_t;
    pub fn semphore_destroy(task: task_t, semaphore: semaphore_t) -> kern_return_t;
    pub fn task_policy_set(
        task: task_policy_set_t,
        flavor: task_policy_flavor_t,
        policy_info: task_policy_t,
        policy_infoCnt: mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_policy_get(
        task: task_policy_set_t,
        flavor: task_policy_flavor_t,
        policy_info: task_policy_t,
        policy_infoCnt: *mut mach_msg_type_number_t,
        get_default: *mut boolean_t,
    ) -> kern_return_t;
    pub fn task_sample(task: task_t, reply: mach_port_t) -> kern_return_t;
    pub fn task_policy(
        task: task_t,
        policy: policy_t,
        base: policy_base_t,
        baseCnt: mach_msg_type_number_t,
        set_limit: boolean_t,
        change: boolean_t,
    ) -> kern_return_t;
    pub fn task_set_emulation(
        target_port: task_t,
        vector_start: *mut c_int,
        emulation_vector: *mut emulation_vector_t,
    ) -> kern_return_t;
    pub fn task_get_emulation_vector(
        task: task_t,
        vector_start: *mut c_int,
        emulation_vector: *mut emulation_vector_t,
        emulation_vectorCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_set_emulation_vector(
        task: task_t,
        vector_start: c_int,
        emulation_vector: emulation_vector_t,
        emulation_vectorCnt: mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_set_ras_pc(
        target_task: task_t,
        basepc: vm_address_t,
        boundspc: vm_address_t,
    ) -> kern_return_t;
    // pub fn task_zone_info(
    //     target_task: task_inspect_t,
    //     names: *mut mach_zone_name_array_t,
    //     namesCnt: *mut mach_msg_type_number_t,
    //     info: *mut task_zone_info_array_t,
    //     infoCnt: *mut mach_msg_type_number_t,
    // ) -> kern_return_t;
    pub fn task_assign(
        task: task_t,
        new_set: processor_set_t,
        assign_threads: boolean_t,
    ) -> kern_return_t;
    pub fn task_assign_default(task: task_t, assign_threads: boolean_t) -> kern_return_t;
    pub fn task_get_assignment(
        task: task_inspect_t,
        assigned_set: *mut processor_set_name_t,
    ) -> kern_return_t;
    pub fn task_set_policy(
        task: task_t,
        pset: processor_set_t,
        policy: policy_t,
        base: policy_base_t,
        baseCnt: mach_msg_type_number_t,
        limit: policy_limit_t,
        limitCnt: mach_msg_type_number_t,
        change: boolean_t,
    ) -> kern_return_t;
    pub fn task_get_state(
        task: task_read_t,
        flavor: thread_state_flavor_t,
        old_state: thread_state_t,
        old_stateCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_set_state(
        task: task_t,
        flavor: thread_state_flavor_t,
        new_state: thread_state_t,
        new_stateCnt: mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_set_phys_footprint_limit(
        task: task_t,
        new_limit: c_int,
        old_limit: *mut c_int,
    ) -> kern_return_t;
    pub fn task_suspend2(
        target_task: task_read_t,
        suspend_token: *mut task_suspension_token_t,
    ) -> kern_return_t;
    pub fn task_resume2(suspend_token: task_suspension_token_t) -> kern_return_t;
    pub fn task_purgable_info(
        task: task_inspect_t,
        stats: *mut task_purgable_info_t,
    ) -> kern_return_t;
    pub fn task_get_mach_voucher(
        task: task_read_t,
        which: mach_voucher_selector_t,
        voucher: *mut ipc_voucher_t,
    ) -> kern_return_t;
    pub fn task_set_mach_voucher(task: task_t, voucher: ipc_voucher_t) -> kern_return_t;
    pub fn task_swap_mach_voucher(
        task: task_t,
        new_voucher: ipc_voucher_t,
        old_voucher: *mut ipc_voucher_t,
    ) -> kern_return_t;
    pub fn task_generate_corpse(
        task: task_read_t,
        corpse_task_port: *mut mach_port_t,
    ) -> kern_return_t;
    pub fn task_map_corpse_info(
        task: task_t,
        corpse_task: task_read_t,
        kcd_addr_begin: *mut vm_address_t,
        kcd_size: *mut u32,
    ) -> kern_return_t;
    pub fn task_register_dyld_image_infos(
        task: task_t,
        dyld_images: dyld_kernel_image_info_array_t,
        dyld_imagesCnt: mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_unregister_dyld_image_infos(
        task: task_t,
        dyld_images: dyld_kernel_image_info_array_t,
        dyld_imagesCnt: mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_get_dyld_image_infos(
        task: task_t,
        dyld_images: *mut dyld_kernel_image_info_array_t,
        dyld_imagesCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_register_dyld_shared_cache_image_info(
        task: task_t,
        dyld_cache_image: dyld_kernel_image_info_t,
        no_cache: boolean_t,
        private_cache: boolean_t,
    ) -> kern_return_t;
    pub fn task_register_dyld_set_dyld_state(task: task_t, dyld_state: u8) -> kern_return_t;
    pub fn task_register_dyld_get_process_state(
        task: task_t,
        dyld_process_state: *mut dyld_kernel_process_info_t,
    ) -> kern_return_t;
    pub fn task_map_corpse_info_64(
        task: task_t,
        corpse_task: task_read_t,
        kcd_addr_begin: *mut mach_vm_address_t,
        kcd_size: *mut mach_vm_size_t,
    ) -> kern_return_t;
    pub fn task_inspect(
        task: task_inspect_t,
        flavor: task_inspect_flavor_t,
        info_out: task_inspect_info_t,
        info_outCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_get_exc_guard_behavior(
        task: task_inspect_t,
        behavior: *mut task_exc_guard_behavior_t,
    ) -> kern_return_t;
    pub fn task_set_exc_guard_behavior(
        task: task_inspect_t,
        behavior: task_exc_guard_behavior_t,
    ) -> kern_return_t;
    pub fn task_dyld_process_info_notify_register(
        target_task: task_read_t,
        notify: mach_port_t,
    ) -> kern_return_t;
    pub fn task_create_identity_token(task: task_t, token: *mut task_id_token_t) -> kern_return_t;
    pub fn task_identity_token_get_task_port(
        token: task_id_token_t,
        flavor: task_flavor_t,
        task_port: *mut mach_port_t,
    ) -> kern_return_t;
    pub fn task_dyld_process_info_notify_deregister(
        target_task: task_read_t,
        notify: mach_port_name_t,
    ) -> kern_return_t;
    pub fn task_get_exception_ports_info(
        port: mach_port_t,
        exception_mask: exception_mask_t,
        masks: exception_mask_array_t,
        masksCnt: *mut mach_msg_type_number_t,
        old_handlers_info: exception_handler_info_array_t,
        old_behaviors: exception_behavior_array_t,
        old_flavors: exception_flavor_array_t,
    ) -> kern_return_t;
    pub fn task_test_sync_upcall(task: task_t, port: mach_port_t) -> kern_return_t;
    pub fn task_set_corpse_forking_behavior(
        task: task_t,
        behavior: task_corpse_forking_behavior_t,
    ) -> kern_return_t;
    pub fn task_test_async_upcall_propagation(
        task: task_t,
        port: mach_port_t,
        qos: c_int,
        iotier: c_int,
    ) -> kern_return_t;
    pub fn task_map_kcdata_object_64(
        task: task_t,
        kcdata_object: kcdata_object_t,
        kcd_addr_begin: *mut mach_vm_address_t,
        kcd_size: *mut mach_vm_size_t,
    ) -> kern_return_t;
    pub fn task_register_hardened_exception_handler(
        task: task_t,
        signed_pc_key: u32,
        exceptions_allowed: exception_mask_t,
        behaviors_allowed: exception_behavior_t,
        flavors_allowed: thread_state_flavor_t,
        new_exception_port: mach_port_t,
    ) -> kern_return_t;
}
