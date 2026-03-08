use anchor_lang::prelude::*;

declare_id!("Bm8qjDWwRgcjVMEQf8uFk3zjKwjgu8x9S173ciYPHqwb");

#[program]
pub mod gym_tracker {
    use super::*;

    // Crear perfil del usuario
    pub fn crear_perfil(ctx: Context<CrearPerfil>, nombre: String) -> Result<()> {
        let perfil = &mut ctx.accounts.perfil;
        perfil.nombre = nombre;
        perfil.press_banca_peso = 0;
        perfil.press_banca_series = 0;
        perfil.sentadilla_peso = 0;
        perfil.sentadilla_series = 0;
        msg!("✅ Perfil creado!");
        Ok(())
    }

    // Registrar press de banca
    pub fn registrar_press_banca(
        ctx: Context<ActualizarPerfil>,
        peso: u32,
        series: u8,
    ) -> Result<()> {
        let perfil = &mut ctx.accounts.perfil;
        perfil.press_banca_peso = peso;
        perfil.press_banca_series = series;
        msg!("🏋️ Press de Banca: {} kg x {} series", peso, series);
        Ok(())
    }

    // Registrar sentadilla
    pub fn registrar_sentadilla(
        ctx: Context<ActualizarPerfil>,
        peso: u32,
        series: u8,
    ) -> Result<()> {
        let perfil = &mut ctx.accounts.perfil;
        perfil.sentadilla_peso = peso;
        perfil.sentadilla_series = series;
        msg!("🦵 Sentadilla: {} kg x {} series", peso, series);
        Ok(())
    }

    // Ver perfil
    pub fn ver_perfil(ctx: Context<VerPerfil>) -> Result<()> {
        let perfil = &ctx.accounts.perfil;
        msg!("============================");
        msg!("👤 Nombre: {}", perfil.nombre);
        msg!("🏋️ Press de Banca: {} kg x {} series", perfil.press_banca_peso, perfil.press_banca_series);
        msg!("🦵 Sentadilla: {} kg x {} series", perfil.sentadilla_peso, perfil.sentadilla_series);
        msg!("============================");
        Ok(())
    }

    // Resetear perfil (poner todo a 0)
    pub fn resetear_perfil(ctx: Context<ActualizarPerfil>) -> Result<()> {
        let perfil = &mut ctx.accounts.perfil;
        perfil.press_banca_peso = 0;
        perfil.press_banca_series = 0;
        perfil.sentadilla_peso = 0;
        perfil.sentadilla_series = 0;
        msg!("🗑️ Perfil reseteado! Todo en 0");
        Ok(())
    }
}

// Estructura del perfil
#[account]
pub struct Perfil {
    pub nombre: String,
    pub press_banca_peso: u32,
    pub press_banca_series: u8,
    pub sentadilla_peso: u32,
    pub sentadilla_series: u8,
}

// Contexto para crear perfil
#[derive(Accounts)]
pub struct CrearPerfil<'info> {
    #[account(
        init,
        payer = usuario,
        space = 8 + 36 + 4 + 1 + 4 + 1,
        seeds = [b"perfil", usuario.key().as_ref()],
        bump,
    )]
    pub perfil: Account<'info, Perfil>,

    #[account(mut)]
    pub usuario: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Contexto para actualizar y resetear perfil
#[derive(Accounts)]
pub struct ActualizarPerfil<'info> {
    #[account(
        mut,
        seeds = [b"perfil", usuario.key().as_ref()],
        bump,
    )]
    pub perfil: Account<'info, Perfil>,

    pub usuario: Signer<'info>,
}

// Contexto para ver perfil (solo lectura)
#[derive(Accounts)]
pub struct VerPerfil<'info> {
    #[account(
        seeds = [b"perfil", usuario.key().as_ref()],
        bump,
    )]
    pub perfil: Account<'info, Perfil>,

    pub usuario: Signer<'info>,
}
