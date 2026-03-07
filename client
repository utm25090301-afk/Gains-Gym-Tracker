// GymTracker - Client

// Obtener el programa
const program = pg.program;
const wallet = pg.wallet;

// Crear la dirección PDA del perfil
const [perfilPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("perfil"), wallet.publicKey.toBuffer()],
  program.programId
);

console.log("👤 Tu wallet:", wallet.publicKey.toString());
console.log("📍 Tu perfil PDA:", perfilPda.toString());

// ==========================================
// 1. CREAR PERFIL
// ==========================================
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
  console.log("📝 Transacción:", tx);
}

// ==========================================
// 2. REGISTRAR PRESS DE BANCA
// ==========================================
async function registrarPressBanca(peso: number, series: number) {
  console.log("\n🏋️ Registrando Press de Banca...");
  
  const tx = await program.methods
    .registrarPressBanca(peso, series)
    .accounts({
      perfil: perfilPda,
      usuario: wallet.publicKey,
    })
    .rpc();

  console.log("✅ Press de Banca registrado!");
  console.log("   Peso:", peso, "kg");
  console.log("   Series:", series);
  console.log("📝 Transacción:", tx);
}

// ==========================================
// 3. REGISTRAR SENTADILLA
// ==========================================
async function registrarSentadilla(peso: number, series: number) {
  console.log("\n🦵 Registrando Sentadilla...");
  
  const tx = await program.methods
    .registrarSentadilla(peso, series)
    .accounts({
      perfil: perfilPda,
      usuario: wallet.publicKey,
    })
    .rpc();

  console.log("✅ Sentadilla registrada!");
  console.log("   Peso:", peso, "kg");
  console.log("   Series:", series);
  console.log("📝 Transacción:", tx);
}

// ==========================================
// 4. VER PERFIL
// ==========================================
async function verPerfil() {
  console.log("\n📊 Obteniendo datos del perfil...");
  
  const perfil = await program.account.perfil.fetch(perfilPda);

  console.log("============================");
  console.log("👤 Nombre:", perfil.nombre);
  console.log("🏋️ Press de Banca:", perfil.pressBancaPeso, "kg x", perfil.pressBancaSeries, "series");
  console.log("🦵 Sentadilla:", perfil.sentadillaPeso, "kg x", perfil.sentadillaSeries, "series");
  console.log("============================");
}

// ==========================================
// EJECUTAR
// ==========================================

// Cambia los valores y descomenta lo que quieras probar:

await crearPerfil("Isma");

// await registrarPressBanca(80, 4);

// await registrarSentadilla(100, 5);

// await verPerfil();
