//! This package provides bindings to the PAPI performance counters
//! library.

#![allow(non_camel_case_types)]

extern crate libc;
extern crate rand;

use std::mem;
use std::sync::{MutexGuard};

#[link(name="stdc++")]
extern {}

#[link(name="papi")]
extern {
    fn PAPI_is_initialized() -> libc::c_int;
    fn PAPI_num_counters() -> libc::c_int;
    fn PAPI_start_counters(events: *const libc::c_int, len: libc::c_int)
        -> libc::c_int;
    fn PAPI_stop_counters(events: *const libc::c_longlong, len: libc::c_int)
        -> libc::c_int;
    fn PAPI_read_counters(values: *mut libc::c_longlong, len: libc::c_int)
        -> libc::c_int;
    fn PAPI_accum_counters(values: *mut libc::c_longlong, len: libc::c_int)
        -> libc::c_int;
}

fn check_status(status: libc::c_int) {
    if status != PAPI_OK {
        panic!("status of papi: {}", status)
    }
}

pub fn is_initialized() -> bool {
    let result = unsafe {PAPI_is_initialized()};
    result != 0
}

pub fn num_counters() -> isize {
	unsafe{
		PAPI_num_counters() as isize
	}
}

fn start_counters(events: &[libc::c_int]) {
    let status = unsafe {
        PAPI_start_counters(events.as_ptr(),
                            events.len() as libc::c_int)
    };
    check_status(status);
}

fn stop_counters(values: &[libc::c_longlong]) {
    let status = unsafe {
        PAPI_stop_counters(values.as_ptr(),
                           values.len() as libc::c_int)
    };
    check_status(status);
}

fn read_counters(values: &mut [libc::c_longlong]) {
    let status = unsafe {
        PAPI_read_counters(mem::transmute(values.as_ptr()),
                           values.len() as libc::c_int)
    };
    check_status(status);
}

fn accum_counters(values: &mut [libc::c_longlong]) {
    let status = unsafe {
        PAPI_accum_counters(mem::transmute(values.as_ptr()),
                            values.len() as libc::c_int)
    };
    check_status(status);
}

pub struct CounterSet {
    counters: Vec<Counter>,
    raw_counters: Vec<libc::c_int>,
    values: Vec<libc::c_longlong>,
}

impl CounterSet {
    pub unsafe fn new(counters: &[Counter]) -> CounterSet {
        let raw_counters
            = counters.iter().map(|x| (*x).clone() as libc::c_int)
            .collect::<Vec<i32>>();
        let values = counters.iter().map(|_| 0i64).collect();
        start_counters(&raw_counters[..]);
        CounterSet {
            counters: Vec::from(counters),
            raw_counters,
            values,
        }
    }

    pub fn read(&mut self) -> Vec<i64> {
        read_counters(&mut self.values[..]);
        self.values.clone()
    }

    pub fn accum(&mut self) -> Vec<i64> {
        accum_counters(&mut self.values[..]);
        self.values.clone()
    }
}

impl<'a> Drop for CounterSet {
    fn drop(&mut self) {
        stop_counters(&mut self.values[..]);
    }
}

// adapted from papiStdEventDefs.h
#[derive(Clone)]
pub enum Counter
{
	PAPI_L1_DCM = 0x80000000,  /*Level 1 data cache misses */
	PAPI_L1_ICM,		 /*Level 1 instruction cache misses */
	PAPI_L2_DCM,		 /*Level 2 data cache misses */
	PAPI_L2_ICM,		 /*Level 2 instruction cache misses */
	PAPI_L3_DCM,		 /*Level 3 data cache misses */
	PAPI_L3_ICM,		 /*Level 3 instruction cache misses */
	PAPI_L1_TCM,		 /*Level 1 total cache misses */
	PAPI_L2_TCM,		 /*Level 2 total cache misses */
	PAPI_L3_TCM,		 /*Level 3 total cache misses */
	PAPI_CA_SNP,		 /*Snoops */
	PAPI_CA_SHR,		 /*Request for shared cache line (SMP) */
	PAPI_CA_CLN,		 /*Request for clean cache line (SMP) */
	PAPI_CA_INV,		 /*Request for cache line Invalidation (SMP) */
	PAPI_CA_ITV,		 /*Request for cache line Intervention (SMP) */
	PAPI_L3_LDM,		 /*Level 3 load misses */
	PAPI_L3_STM,		 /*Level 3 store misses */
/* 0x10 */
	PAPI_BRU_IDL,		 /*Cycles branch units are idle */
	PAPI_FXU_IDL,		 /*Cycles integer units are idle */
	PAPI_FPU_IDL,		 /*Cycles floating point units are idle */
	PAPI_LSU_IDL,		 /*Cycles load/store units are idle */
	PAPI_TLB_DM,		 /*Data translation lookaside buffer misses */
	PAPI_TLB_IM,		 /*Instr translation lookaside buffer misses */
	PAPI_TLB_TL,		 /*Total translation lookaside buffer misses */
	PAPI_L1_LDM,		 /*Level 1 load misses */
	PAPI_L1_STM,		 /*Level 1 store misses */
	PAPI_L2_LDM,		 /*Level 2 load misses */
	PAPI_L2_STM,		 /*Level 2 store misses */
	PAPI_BTAC_M,		 /*BTAC miss */
	PAPI_PRF_DM,		 /*Prefetch data instruction caused a miss */
	PAPI_L3_DCH,		 /*Level 3 Data Cache Hit */
	PAPI_TLB_SD,		 /*Xlation lookaside buffer shootdowns (SMP) */
	PAPI_CSR_FAL,		 /*Failed store conditional instructions */
/* 0x20 */
	PAPI_CSR_SUC,		 /*Successful store conditional instructions */
	PAPI_CSR_TOT,		 /*Total store conditional instructions */
	PAPI_MEM_SCY,		 /*Cycles Stalled Waiting for Memory Access */
	PAPI_MEM_RCY,		 /*Cycles Stalled Waiting for Memory Read */
	PAPI_MEM_WCY,		 /*Cycles Stalled Waiting for Memory Write */
	PAPI_STL_ICY,		 /*Cycles with No Instruction Issue */
	PAPI_FUL_ICY,		 /*Cycles with Maximum Instruction Issue */
	PAPI_STL_CCY,		 /*Cycles with No Instruction Completion */
	PAPI_FUL_CCY,		 /*Cycles with Maximum Instruction Completion */
	PAPI_HW_INT,		 /*Hardware interrupts */
	PAPI_BR_UCN,		 /*Unconditional branch instructions executed */
	PAPI_BR_CN,			 /*Conditional branch instructions executed */
	PAPI_BR_TKN,		 /*Conditional branch instructions taken */
	PAPI_BR_NTK,		 /*Conditional branch instructions not taken */
	PAPI_BR_MSP,		 /*Conditional branch instructions mispred */
	PAPI_BR_PRC,		 /*Conditional branch instructions corr. pred */
/* 0x30 */
	PAPI_FMA_INS,		 /*FMA instructions completed */
	PAPI_TOT_IIS,		 /*Total instructions issued */
	PAPI_TOT_INS,		 /*Total instructions executed */
	PAPI_INT_INS,		 /*Integer instructions executed */
	PAPI_FP_INS,		 /*Floating point instructions executed */
	PAPI_LD_INS,		 /*Load instructions executed */
	PAPI_SR_INS,		 /*Store instructions executed */
	PAPI_BR_INS,		 /*Total branch instructions executed */
	PAPI_VEC_INS,		 /*Vector/SIMD instructions executed (could include integer) */
	PAPI_RES_STL,		 /*Cycles processor is stalled on resource */
	PAPI_FP_STAL,		 /*Cycles any FP units are stalled */
	PAPI_TOT_CYC,		 /*Total cycles executed */
	PAPI_LST_INS,		 /*Total load/store inst. executed */
	PAPI_SYC_INS,		 /*Sync. inst. executed */
	PAPI_L1_DCH,		 /*L1 D Cache Hit */
	PAPI_L2_DCH,		 /*L2 D Cache Hit */
	/* 0x40 */
	PAPI_L1_DCA,		 /*L1 D Cache Access */
	PAPI_L2_DCA,		 /*L2 D Cache Access */
	PAPI_L3_DCA,		 /*L3 D Cache Access */
	PAPI_L1_DCR,		 /*L1 D Cache Read */
	PAPI_L2_DCR,		 /*L2 D Cache Read */
	PAPI_L3_DCR,		 /*L3 D Cache Read */
	PAPI_L1_DCW,		 /*L1 D Cache Write */
	PAPI_L2_DCW,		 /*L2 D Cache Write */
	PAPI_L3_DCW,		 /*L3 D Cache Write */
	PAPI_L1_ICH,		 /*L1 instruction cache hits */
	PAPI_L2_ICH,		 /*L2 instruction cache hits */
	PAPI_L3_ICH,		 /*L3 instruction cache hits */
	PAPI_L1_ICA,		 /*L1 instruction cache accesses */
	PAPI_L2_ICA,		 /*L2 instruction cache accesses */
	PAPI_L3_ICA,		 /*L3 instruction cache accesses */
	PAPI_L1_ICR,		 /*L1 instruction cache reads */
	/* 0x50 */
	PAPI_L2_ICR,		 /*L2 instruction cache reads */
	PAPI_L3_ICR,		 /*L3 instruction cache reads */
	PAPI_L1_ICW,		 /*L1 instruction cache writes */
	PAPI_L2_ICW,		 /*L2 instruction cache writes */
	PAPI_L3_ICW,		 /*L3 instruction cache writes */
	PAPI_L1_TCH,		 /*L1 total cache hits */
	PAPI_L2_TCH,		 /*L2 total cache hits */
	PAPI_L3_TCH,		 /*L3 total cache hits */
	PAPI_L1_TCA,		 /*L1 total cache accesses */
	PAPI_L2_TCA,		 /*L2 total cache accesses */
	PAPI_L3_TCA,		 /*L3 total cache accesses */
	PAPI_L1_TCR,		 /*L1 total cache reads */
	PAPI_L2_TCR,		 /*L2 total cache reads */
	PAPI_L3_TCR,		 /*L3 total cache reads */
	PAPI_L1_TCW,		 /*L1 total cache writes */
	PAPI_L2_TCW,		 /*L2 total cache writes */
	/* 0x60 */
	PAPI_L3_TCW,		 /*L3 total cache writes */
	PAPI_FML_INS,		 /*FM ins */
	PAPI_FAD_INS,		 /*FA ins */
	PAPI_FDV_INS,		 /*FD ins */
	PAPI_FSQ_INS,		 /*FSq ins */
	PAPI_FNV_INS,		 /*Finv ins */
	PAPI_FP_OPS,		 /*Floating point operations executed */
	PAPI_SP_OPS,		 /* Floating point operations executed; optimized to count scaled single precision vector operations */
	PAPI_DP_OPS,		 /* Floating point operations executed; optimized to count scaled double precision vector operations */
	PAPI_VEC_SP,		 /* Single precision vector/SIMD instructions */
	PAPI_VEC_DP,		 /* Double precision vector/SIMD instructions */
	PAPI_REF_CYC,		 /* Reference clock cycles */
	PAPI_END			 /*This should always be last! */
}

// Return codes
// adapted from papi.h
/** No error */
pub static PAPI_OK        : libc::c_int =  0  ;   
/** Invalid argument */
pub static PAPI_EINVAL    : libc::c_int = -1  ;   
/** Insufficient memory */
pub static PAPI_ENOMEM    : libc::c_int = -2  ;   
/** A System/C library call failed */
pub static PAPI_ESYS      : libc::c_int = -3  ;   
/** Not supported by component */
pub static PAPI_ECMP      : libc::c_int = -4  ;   
/** Backwards compatibility */
pub static PAPI_ESBSTR    : libc::c_int = -4  ;   
/** Access to the counters was lost or interrupted */
pub static PAPI_ECLOST    : libc::c_int = -5  ;   
/** Internal error, please send mail to the developers */
pub static PAPI_EBUG      : libc::c_int = -6  ;   
/** Event does not exist */
pub static PAPI_ENOEVNT   : libc::c_int = -7  ;   
/** Event exists, but cannot be counted due to counter resource limitations */
pub static PAPI_ECNFLCT   : libc::c_int = -8  ;   
/** EventSet is currently not running */
pub static PAPI_ENOTRUN   : libc::c_int = -9  ;   
/** EventSet is currently counting */
pub static PAPI_EISRUN    : libc::c_int = -10 ;   
/** No such EventSet Available */
pub static PAPI_ENOEVST   : libc::c_int = -11 ;   
/** Event in argument is not a valid preset */
pub static PAPI_ENOTPRESET: libc::c_int = -12 ;   
/** Hardware does not support performance counters */
pub static PAPI_ENOCNTR   : libc::c_int = -13 ;   
/** Unknown error code */
pub static PAPI_EMISC     : libc::c_int = -14 ;   
/** Permission level does not permit operation */
pub static PAPI_EPERM     : libc::c_int = -15 ;   
/** PAPI hasn't been initialized yet */
pub static PAPI_ENOINIT   : libc::c_int = -16 ;   
/** Component Index isn't set */
pub static PAPI_ENOCMP    : libc::c_int = -17 ;   
/** Not supported */
pub static PAPI_ENOSUPP   : libc::c_int = -18 ;   
/** Not implemented */
pub static PAPI_ENOIMPL   : libc::c_int = -19 ;   
/** Buffer size exceeded */
pub static PAPI_EBUF      : libc::c_int = -20 ;   
/** EventSet domain is not supported for the operation */
pub static PAPI_EINVAL_DOM: libc::c_int = -21 ;   
/** Invalid or missing event attributes */
pub static PAPI_EATTR	  : libc::c_int =  -22;    
/** Too many events or attributes */
pub static PAPI_ECOUNT	  : libc::c_int =  -23;    
/** Bad combination of features */
pub static PAPI_ECOMBO	  : libc::c_int =  -24;    
/** Number of error messages specified in this API */
pub static PAPI_NUM_ERRORS: libc::c_int =   25;    
