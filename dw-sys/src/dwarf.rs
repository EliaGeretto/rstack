use libc::{c_int, c_uint};

pub const DW_TAG_array_type: c_int = 0x01;
pub const DW_TAG_class_type: c_int = 0x02;
pub const DW_TAG_entry_point: c_int = 0x03;
pub const DW_TAG_enumeration_type: c_int = 0x04;
pub const DW_TAG_formal_parameter: c_int = 0x05;
pub const DW_TAG_imported_declaration: c_int = 0x08;
pub const DW_TAG_label: c_int = 0x0a;
pub const DW_TAG_lexical_block: c_int = 0x0b;
pub const DW_TAG_member: c_int = 0x0d;
pub const DW_TAG_pointer_type: c_int = 0x0f;
pub const DW_TAG_reference_type: c_int = 0x10;
pub const DW_TAG_compile_unit: c_int = 0x11;
pub const DW_TAG_string_type: c_int = 0x12;
pub const DW_TAG_structure_type: c_int = 0x13;
pub const DW_TAG_subroutine_type: c_int = 0x15;
pub const DW_TAG_typedef: c_int = 0x16;
pub const DW_TAG_union_type: c_int = 0x17;
pub const DW_TAG_unspecified_parameters: c_int = 0x18;
pub const DW_TAG_variant: c_int = 0x19;
pub const DW_TAG_common_block: c_int = 0x1a;
pub const DW_TAG_common_inclusion: c_int = 0x1b;
pub const DW_TAG_inheritance: c_int = 0x1c;
pub const DW_TAG_inlined_subroutine: c_int = 0x1d;
pub const DW_TAG_module: c_int = 0x1e;
pub const DW_TAG_ptr_to_member_type: c_int = 0x1f;
pub const DW_TAG_set_type: c_int = 0x20;
pub const DW_TAG_subrange_type: c_int = 0x21;
pub const DW_TAG_with_stmt: c_int = 0x22;
pub const DW_TAG_access_declaration: c_int = 0x23;
pub const DW_TAG_base_type: c_int = 0x24;
pub const DW_TAG_catch_block: c_int = 0x25;
pub const DW_TAG_const_type: c_int = 0x26;
pub const DW_TAG_constant: c_int = 0x27;
pub const DW_TAG_enumerator: c_int = 0x28;
pub const DW_TAG_file_type: c_int = 0x29;
pub const DW_TAG_friend: c_int = 0x2a;
pub const DW_TAG_namelist: c_int = 0x2b;
pub const DW_TAG_namelist_item: c_int = 0x2c;
pub const DW_TAG_packed_type: c_int = 0x2d;
pub const DW_TAG_subprogram: c_int = 0x2e;
pub const DW_TAG_template_type_parameter: c_int = 0x2f;
pub const DW_TAG_template_value_parameter: c_int = 0x30;
pub const DW_TAG_thrown_type: c_int = 0x31;
pub const DW_TAG_try_block: c_int = 0x32;
pub const DW_TAG_variant_part: c_int = 0x33;
pub const DW_TAG_variable: c_int = 0x34;
pub const DW_TAG_volatile_type: c_int = 0x35;
pub const DW_TAG_dwarf_procedure: c_int = 0x36;
pub const DW_TAG_restrict_type: c_int = 0x37;
pub const DW_TAG_interface_type: c_int = 0x38;
pub const DW_TAG_namespace: c_int = 0x39;
pub const DW_TAG_imported_module: c_int = 0x3a;
pub const DW_TAG_unspecified_type: c_int = 0x3b;
pub const DW_TAG_partial_unit: c_int = 0x3c;
pub const DW_TAG_imported_unit: c_int = 0x3d;
pub const DW_TAG_condition: c_int = 0x3f;
pub const DW_TAG_shared_type: c_int = 0x40;
pub const DW_TAG_type_unit: c_int = 0x41;
pub const DW_TAG_rvalue_reference_type: c_int = 0x42;
pub const DW_TAG_template_alias: c_int = 0x43;
pub const DW_TAG_atomic_type: c_int = 0x47;
pub const DW_TAG_lo_user: c_int = 0x4080;
pub const DW_TAG_MIPS_loop: c_int = 0x4081;
pub const DW_TAG_format_label: c_int = 0x4101;
pub const DW_TAG_function_template: c_int = 0x4102;
pub const DW_TAG_class_template: c_int = 0x4103;
pub const DW_TAG_GNU_BINCL: c_int = 0x4104;
pub const DW_TAG_GNU_EINCL: c_int = 0x4105;
pub const DW_TAG_GNU_template_template_param: c_int = 0x4106;
pub const DW_TAG_GNU_template_parameter_pack: c_int = 0x4107;
pub const DW_TAG_GNU_formal_parameter_pack: c_int = 0x4108;
pub const DW_TAG_GNU_call_site: c_int = 0x4109;
pub const DW_TAG_GNU_call_site_parameter: c_int = 0x410a;
pub const DW_TAG_hi_user: c_int = 0xffff;

pub const DW_AT_sibling: c_uint = 0x01;
pub const DW_AT_location: c_uint = 0x02;
pub const DW_AT_name: c_uint = 0x03;
pub const DW_AT_ordering: c_uint = 0x09;
pub const DW_AT_byte_size: c_uint = 0x0b;
pub const DW_AT_bit_offset: c_uint = 0x0c;
pub const DW_AT_bit_size: c_uint = 0x0d;
pub const DW_AT_stmt_list: c_uint = 0x10;
pub const DW_AT_low_pc: c_uint = 0x11;
pub const DW_AT_high_pc: c_uint = 0x12;
pub const DW_AT_language: c_uint = 0x13;
pub const DW_AT_discr: c_uint = 0x15;
pub const DW_AT_discr_value: c_uint = 0x16;
pub const DW_AT_visibility: c_uint = 0x17;
pub const DW_AT_import: c_uint = 0x18;
pub const DW_AT_string_length: c_uint = 0x19;
pub const DW_AT_common_reference: c_uint = 0x1a;
pub const DW_AT_comp_dir: c_uint = 0x1b;
pub const DW_AT_const_value: c_uint = 0x1c;
pub const DW_AT_containing_type: c_uint = 0x1d;
pub const DW_AT_default_value: c_uint = 0x1e;
pub const DW_AT_inline: c_uint = 0x20;
pub const DW_AT_is_optional: c_uint = 0x21;
pub const DW_AT_lower_bound: c_uint = 0x22;
pub const DW_AT_producer: c_uint = 0x25;
pub const DW_AT_prototyped: c_uint = 0x27;
pub const DW_AT_return_addr: c_uint = 0x2a;
pub const DW_AT_start_scope: c_uint = 0x2c;
pub const DW_AT_bit_stride: c_uint = 0x2e;
pub const DW_AT_upper_bound: c_uint = 0x2f;
pub const DW_AT_abstract_origin: c_uint = 0x31;
pub const DW_AT_accessibility: c_uint = 0x32;
pub const DW_AT_address_class: c_uint = 0x33;
pub const DW_AT_artificial: c_uint = 0x34;
pub const DW_AT_base_types: c_uint = 0x35;
pub const DW_AT_calling_convention: c_uint = 0x36;
pub const DW_AT_count: c_uint = 0x37;
pub const DW_AT_data_member_location: c_uint = 0x38;
pub const DW_AT_decl_column: c_uint = 0x39;
pub const DW_AT_decl_file: c_uint = 0x3a;
pub const DW_AT_decl_line: c_uint = 0x3b;
pub const DW_AT_declaration: c_uint = 0x3c;
pub const DW_AT_discr_list: c_uint = 0x3d;
pub const DW_AT_encoding: c_uint = 0x3e;
pub const DW_AT_external: c_uint = 0x3f;
pub const DW_AT_frame_base: c_uint = 0x40;
pub const DW_AT_friend: c_uint = 0x41;
pub const DW_AT_identifier_case: c_uint = 0x42;
pub const DW_AT_macro_info: c_uint = 0x43;
pub const DW_AT_namelist_item: c_uint = 0x44;
pub const DW_AT_priority: c_uint = 0x45;
pub const DW_AT_segment: c_uint = 0x46;
pub const DW_AT_specification: c_uint = 0x47;
pub const DW_AT_static_link: c_uint = 0x48;
pub const DW_AT_type: c_uint = 0x49;
pub const DW_AT_use_location: c_uint = 0x4a;
pub const DW_AT_variable_parameter: c_uint = 0x4b;
pub const DW_AT_virtuality: c_uint = 0x4c;
pub const DW_AT_vtable_elem_location: c_uint = 0x4d;
pub const DW_AT_allocated: c_uint = 0x4e;
pub const DW_AT_associated: c_uint = 0x4f;
pub const DW_AT_data_location: c_uint = 0x50;
pub const DW_AT_byte_stride: c_uint = 0x51;
pub const DW_AT_entry_pc: c_uint = 0x52;
pub const DW_AT_use_UTF8: c_uint = 0x53;
pub const DW_AT_extension: c_uint = 0x54;
pub const DW_AT_ranges: c_uint = 0x55;
pub const DW_AT_trampoline: c_uint = 0x56;
pub const DW_AT_call_column: c_uint = 0x57;
pub const DW_AT_call_file: c_uint = 0x58;
pub const DW_AT_call_line: c_uint = 0x59;
pub const DW_AT_description: c_uint = 0x5a;
pub const DW_AT_binary_scale: c_uint = 0x5b;
pub const DW_AT_decimal_scale: c_uint = 0x5c;
pub const DW_AT_small: c_uint = 0x5d;
pub const DW_AT_decimal_sign: c_uint = 0x5e;
pub const DW_AT_digit_count: c_uint = 0x5f;
pub const DW_AT_picture_string: c_uint = 0x60;
pub const DW_AT_mutable: c_uint = 0x61;
pub const DW_AT_threads_scaled: c_uint = 0x62;
pub const DW_AT_explicit: c_uint = 0x63;
pub const DW_AT_object_pointer: c_uint = 0x64;
pub const DW_AT_endianity: c_uint = 0x65;
pub const DW_AT_elemental: c_uint = 0x66;
pub const DW_AT_pure: c_uint = 0x67;
pub const DW_AT_recursive: c_uint = 0x68;
pub const DW_AT_signature: c_uint = 0x69;
pub const DW_AT_main_subprogram: c_uint = 0x6a;
pub const DW_AT_data_bit_offset: c_uint = 0x6b;
pub const DW_AT_const_expr: c_uint = 0x6c;
pub const DW_AT_enum_class: c_uint = 0x6d;
pub const DW_AT_linkage_name: c_uint = 0x6e;
pub const DW_AT_noreturn: c_uint = 0x87;
pub const DW_AT_lo_user: c_uint = 0x2000;
pub const DW_AT_MIPS_fde: c_uint = 0x2001;
pub const DW_AT_MIPS_loop_begin: c_uint = 0x2002;
pub const DW_AT_MIPS_tail_loop_begin: c_uint = 0x2003;
pub const DW_AT_MIPS_epilog_begin: c_uint = 0x2004;
pub const DW_AT_MIPS_loop_unroll_factor: c_uint = 0x2005;
pub const DW_AT_MIPS_software_pipeline_depth: c_uint = 0x2006;
pub const DW_AT_MIPS_linkage_name: c_uint = 0x2007;
pub const DW_AT_MIPS_stride: c_uint = 0x2008;
pub const DW_AT_MIPS_abstract_name: c_uint = 0x2009;
pub const DW_AT_MIPS_clone_origin: c_uint = 0x200a;
pub const DW_AT_MIPS_has_inlines: c_uint = 0x200b;
pub const DW_AT_MIPS_stride_byte: c_uint = 0x200c;
pub const DW_AT_MIPS_stride_elem: c_uint = 0x200d;
pub const DW_AT_MIPS_ptr_dopetype: c_uint = 0x200e;
pub const DW_AT_MIPS_allocatable_dopetype: c_uint = 0x200f;
pub const DW_AT_MIPS_assumed_shape_dopetype: c_uint = 0x2010;
pub const DW_AT_MIPS_assumed_size: c_uint = 0x2011;
pub const DW_AT_sf_names: c_uint = 0x2101;
pub const DW_AT_src_info: c_uint = 0x2102;
pub const DW_AT_mac_info: c_uint = 0x2103;
pub const DW_AT_src_coords: c_uint = 0x2104;
pub const DW_AT_body_begin: c_uint = 0x2105;
pub const DW_AT_body_end: c_uint = 0x2106;
pub const DW_AT_GNU_vector: c_uint = 0x2107;
pub const DW_AT_GNU_guarded_by: c_uint = 0x2108;
pub const DW_AT_GNU_pt_guarded_by: c_uint = 0x2109;
pub const DW_AT_GNU_guarded: c_uint = 0x210a;
pub const DW_AT_GNU_pt_guarded: c_uint = 0x210b;
pub const DW_AT_GNU_locks_excluded: c_uint = 0x210c;
pub const DW_AT_GNU_exclusive_locks_required: c_uint = 0x210d;
pub const DW_AT_GNU_shared_locks_required: c_uint = 0x210e;
pub const DW_AT_GNU_odr_signature: c_uint = 0x210f;
pub const DW_AT_GNU_template_name: c_uint = 0x2110;
pub const DW_AT_GNU_call_site_value: c_uint = 0x2111;
pub const DW_AT_GNU_call_site_data_value: c_uint = 0x2112;
pub const DW_AT_GNU_call_site_target: c_uint = 0x2113;
pub const DW_AT_GNU_call_site_target_clobbered: c_uint = 0x2114;
pub const DW_AT_GNU_tail_call: c_uint = 0x2115;
pub const DW_AT_GNU_all_tail_call_sites: c_uint = 0x2116;
pub const DW_AT_GNU_all_call_sites: c_uint = 0x2117;
pub const DW_AT_GNU_all_source_call_sites: c_uint = 0x2118;
pub const DW_AT_GNU_macros: c_uint = 0x2119;
pub const DW_AT_GNU_deleted: c_uint = 0x211a;
pub const DW_AT_hi_user: c_uint = 0x3fff;
