use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mycalculatordapp {
    use super::*;

    pub fn create(ctx:Context<Create>, init_message: String) -> ProgramResult {
    }

    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> ProgramResult {
    }

    pub fn multiply(ctx: Context<Multiplication>, num1: i64, num2: i64) -> ProgramResult {
    }

    pub fn subtract(ctx: Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult {
    }

    pub fn divide(ctx: Context<Division>, num1: i64, num2: i64) -> ProgramResult {
    }
    
}