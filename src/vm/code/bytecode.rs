pub const UNKNOWN: u8 = 0;
pub const POP_TOP: u8 = 1;
pub const ROT_TWO: u8 = 2;
pub const ROT_THREE: u8 = 3;
pub const DUP_TOP: u8 = 4;
pub const UNARY_NEGATIVE: u8 = 11;
pub const BINARY_MULTIPLY: u8 = 20;
pub const BINARY_MODULO: u8 = 22;
pub const BINARY_SUBSCR: u8 = 25;
pub const BINARY_DIVIDE: u8 = 21;
pub const BINARY_ADD: u8 = 23;
pub const BINARY_SUBSTRACT: u8 = 24;

pub const INPLACE_ADD: u8 = 55;
pub const STORE_MAP: u8 = 54;
pub const INPLACE_SUBSTRACT: u8 = 56;
pub const INPLACE_MULTIPLY: u8 = 57;
pub const INPLACE_DIVIDE: u8 = 58;
pub const INPLACE_MODULO: u8 = 59;
pub const STORE_SUBSCR: u8 = 60;

pub const GET_ITER: u8 = 68;

pub const PRINT_ITEM: u8 = 71;
pub const PRINT_NEWLINE: u8 = 72;

pub const BREAK_LOOP: u8 = 80;
pub const LOAD_LOCALS: u8 = 82;
pub const RETURN_VALUE: u8 = 83;
pub const POP_BLOCK: u8 = 87;
pub const BUILD_CLASS: u8 = 89;

// TODO: This is a separator
pub const HAVE_ARGUMENT: u8 = 90; /* Opcodes from here have an argument: */

pub const STORE_NAME: u8 = 90; /* Index in name list */
pub const FOR_ITER: u8 = 93;
pub const STORE_ATTR: u8 = 95; /* Index in name list */
pub const STORE_GLOBAL: u8 = 97;
pub const DUP_TOPX: u8 = 99; /* number of items to duplicate */
pub const LOAD_CONST: u8 = 100; /* Index in const list */
pub const LOAD_NAME: u8 = 101; /* Index in name list */
pub const BUILD_LIST: u8 = 103;
pub const BUILD_MAP: u8 = 105;
pub const LOAD_ATTR: u8 = 106; /* Index in name list */
pub const COMPARE_OP: u8 = 107; /* Comparison operator */
pub const JUMP_FORWARD: u8 = 110; /* Number of bytes to skip */
pub const JUMP_IF_FALSE_OR_POP: u8 = 111; /* Target byte offset from beginningof code */

pub const JUMP_ABSOLUTE: u8 = 113;
pub const POP_JUMP_IF_FALSE: u8 = 114;
pub const POP_JUMP_IF_TRUE: u8 = 115;
pub const LOAD_GLOBAL: u8 = 116; /* Index in name list */
pub const SETUP_LOOP: u8 = 120; /* Target address (relative) */
pub const LOAD_FAST: u8 = 124; /* Local variable number */
pub const STORE_FAST: u8 = 125; /* Local variable number */

pub const CALL_FUNCTION: u8 = 131;
pub const MAKE_FUNCTION: u8 = 132;
