const program = pg.program;
const wallet = pg.wallet;

const [perfilPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("perfil"), wallet.publicKey.toBuffer()],
  program.programId
);

// 1. CREAR PERFIL
async function crearPerfil(nombre: string) {
  console.log("\n🏋️ Creando perfil...");
  const tx = await program.methods
    .crearPerfil(nombre)
    .accounts({
      perfil: perfilPda,
      usuario: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
  console.log("✅ Perfil creado!");
}

// 2. REGISTRAR PRESS DE BANCA
async function registrarPressBanca(peso: number, series: number) {
  console.log("\n🏋️ Registrando Press de Banca...");
  const tx = await program.methods
    .registrarPressBanca(peso, series)
    .accounts({
      perfil: perfilPda,
      usuario: wallet.publicKey,
    })
    .rpc();
  console.log("✅ Press de Banca:", peso, "kg x", series, "series");
}

// 3. REGISTRAR SENTADILLA
async function registrarSentadilla(peso: number, series: number) {
  console.log("\n🦵 Registrando Sentadilla...");
  const tx = await program.methods
    .registrarSentadilla(peso, series)
    .accounts({
      perfil: perfilPda,
      usuario: wallet.publicKey,
    })
    .rpc();
  console.log("✅ Sentadilla:", peso, "kg x", series, "series");
}

// 4. VER PERFIL
async function verPerfil() {
  console.log("\n📊 Datos del perfil:");
  const perfil = await program.account.perfil.fetch(perfilPda);
  console.log("============================");
  console.log("👤 Nombre:", perfil.nombre);
  console.log("🏋️ Press de Banca:", perfil.pressBancaPeso, "kg x", perfil.pressBancaSeries, "series");
  console.log("🦵 Sentadilla:", perfil.sentadillaPeso, "kg x", perfil.sentadillaSeries, "series");
  console.log("============================");
}

// 5. RESETEAR PERFIL
async function resetearPerfil() {
  console.log("\n🗑️ Reseteando perfil...");
  const tx = await program.methods
    .resetearPerfil()
    .accounts({
      perfil: perfilPda,
      usuario: wallet.publicKey,
    })
    .rpc();
  console.log("✅ Perfil reseteado! Todo en 0");
}

// ==========================================
// EJECUTAR
// ==========================================

//await crearPerfil("isma");
await registrarPressBanca(80, 4);
await registrarSentadilla(100, 5);
await verPerfil();
await resetearPerfil();
await verPerfil();
