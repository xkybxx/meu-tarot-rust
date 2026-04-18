// biblioteca padrão de entrada e saída, ele vai ler a escolha do usuário no futuro, quando eu quiser que ele escolha o deck.
use std::io; 
// 1. use diz ao Rust que vou usar algo de uma biblioteca externa. O prelude é tipo um kit de ferramentas que traz várias coisas úteis de uma vez só.
// 2. rand é a biblioteca de aleatoriedade, e o prelude traz as funções de embaralhamento e geração de números aleatórios.
//O "prelude" é um pacote que contém as funções mais comuns da biblioteca. O asterisco (*) significa "traga tudo o que estiver aí dentro". É graças a isso que o seu código entende o que é shuffle.
use rand::prelude::*; 
#[derive(Debug, Clone)] //Isso é um "presente" do Rust. O Debug permite que você imprima a carta inteira no terminal para testes, e o Clone permite que o Rust tire cópias dessa carta se precisar.
#[allow(dead_code)] //Código morto (código que existe, mas não é executado)
struct Carta { //Define um novo tipo de dado. É como se você estivesse criando a "ficha técnica" do que compõe uma carta de Tarot.
    nome: String, //String é um tipo de dado que representa texto. Ele é mais flexível que &str (que é uma fatia de string) porque pode crescer e mudar de tamanho, o que é útil para os nomes das cartas.
    numero: u8, //É um número inteiro pequeno (0 a 255). Como o Tarot não tem milhares de cartas, o u8 economiza memória em comparação ao i32
    significado: String, //O significado é o texto que explica o que a carta representa. Ele também é uma String porque pode ser uma frase ou parágrafo, e pode variar muito em tamanho.
}

// enum: Abreviação de "enumeração". Você usa quando uma coisa só pode ser uma entre várias opções fixas. Ou o deck é Rider-Waite, ou é Lenormand. Não tem como ser os dois ao mesmo tempo.
enum EstiloTarot { 
    Lenormand,
    RiderWaite,
    SibillaItaliana,
}

// fn main(): Todo programa Rust começa aqui.
//let escolha_usuario: Você criou uma variável e já "setou" ela como RiderWaite. No futuro, você pode mudar isso para receber um clique do usuário.
fn main() {
      println!("--- ENTRE NO VÉU DA LUA ---");
    println!("Escolha um deck para sua tiragem:");
    println!("1. Lenormand");
    println!("2. Rider-Waite");
    println!("3. Sibilla Italiana (Em breve)");

    // Criamos uma String vazia para guardar o que você digitar
    let mut entrada = String::new();

    // O programa para e espera você digitar algo e apertar ENTER
    io::stdin() // stdin() é a função que representa a entrada padrão (o teclado). O read_line(&mut entrada) lê o que você digitou e coloca dentro da variável "entrada". O &mut é necessário porque a função precisa modificar a variável, e o Rust exige que você seja explícito sobre isso.
        .read_line(&mut entrada)
        .expect("Falha ao ler a entrada");

    // Aqui a mágica acontece: transformamos o "1" ou "2" em um valor do EstiloTarot
    let escolha_usuario = match entrada.trim() {
        "1" => EstiloTarot::Lenormand,
        "2" => EstiloTarot::RiderWaite,
        "3" => EstiloTarot::SibillaItaliana,
        _ => {
            println!("Opção inválida! Escolhendo Rider-Waite por padrão.");
            EstiloTarot::RiderWaite
        }
    };

    match escolha_usuario { //Ele olha para a escolha_usuario e diz: "Se for isso, faça aquilo". O Rust te obriga a tratar todas as opções que você colocou no enum, então se você esquecer de um estilo, ele vai reclamar.
        
//Aqui começa os códigos dos decks, cada braço do match é um deck diferente. O código dentro de cada braço é o que acontece quando o usuário escolhe aquele deck.
        
//Aqui começa o código do Lenormand, eu coloquei as 36 cartas tradicionais.
        EstiloTarot::Lenormand => {
            println!("Preparando sua tiragem de Lenormand...");
            let carta1 = Carta {
                nome: String::from("O Cavaleiro"),
                numero: 1,
                significado: String::from("Notícias e movimento"),
            };
            let carta2 = Carta {
                nome: String::from("O Trevo"),
                numero: 2,
                significado: String::from("Sorte e oportunidades"),
            };
            let carta3 = Carta {
                nome: String::from("O Navio"),
                numero: 3,
                significado: String::from("Viagens e mudanças"),
            };
             let carta4 = Carta {
                nome: String::from("A Casa"),
                numero: 4,
                significado: String::from("Lar e família"),
            };
             let carta5 = Carta {
                nome: String::from("A Árvore"),
                numero: 5,
                significado: String::from("Saúde e crescimento"),
            };
             let carta6 = Carta {
                nome: String::from("As Nuvens"),
                numero: 6,
                significado: String::from("Confusão e incerteza"),
            };
             let carta7 = Carta {
                nome: String::from("A Serpente"),
                numero: 7,
                significado: String::from("Engano e traição"),
            };
             let carta8 = Carta {
                nome: String::from("O Caixão"),
                numero: 8,
                significado: String::from("Fim de ciclo e transformação"),
            };
             let carta9 = Carta {
                nome: String::from("O Buquê"),
                numero: 9,
                significado: String::from("Felicidade e celebração"),
            };
             let carta10 = Carta {
                nome: String::from("A Foice"),
                numero: 10,
                significado: String::from("Cortes e decisões difíceis"),
            };
             let carta11 = Carta {
                nome: String::from("O Chicote"),
                numero: 11,
                significado: String::from("Conflitos e discussões"),
            };
             let carta12 = Carta {
                nome: String::from("Os Pássaros"),
                numero: 12,
                significado: String::from("Comunicação e ansiedade"),
            };
             let carta13 = Carta {
                nome: String::from("A Criança"),
                numero: 13,
                significado: String::from("Inocência e novos começos"),
            };
             let carta14 = Carta {
                nome: String::from("A Raposa"),
                numero: 14,
                significado: String::from("Astúcia e trabalho"),
            };
             let carta15 = Carta {
                nome: String::from("O Urso"),
                numero: 15,
                significado: String::from("Força e proteção"),
            };
             let carta16 = Carta {
                nome: String::from("A Estrela"),
                numero: 16,
                significado: String::from("Esperança e inspiração"),
            };
             let carta17 = Carta {
                nome: String::from("A Cegonha"),
                numero: 17,
                significado: String::from("Mudanças e novidades"),
            };  
             let carta18 = Carta {
                nome: String::from("O Cão"),
                numero: 18,
                significado: String::from("Amizade e lealdade"),
            };
             let carta19 = Carta {
                nome: String::from("A Torre"),
                numero: 19,
                significado: String::from("Isolamento e autoridade"),
            };
             let carta20 = Carta {
                nome: String::from("O Jardim"),
                numero: 20,
                significado: String::from("Socialização e eventos"),
            };
             let carta21 = Carta {
                nome: String::from("A Montanha"),
                numero: 21,
                significado: String::from("Desafios e obstáculos"),
            };
             let carta22 = Carta {
                nome: String::from("O Caminho"),
                numero: 22,
                significado: String::from("Decisões e escolhas"),
            }; 
            let carta23 = Carta {
                    nome: String::from("Os Ratos"),
                    numero: 23,
                    significado: String::from("Perdas e preocupações"),
                };
            let carta24 = Carta {
                    nome: String::from("O Coração"),
                    numero: 24,
                    significado: String::from("Amor e emoções"),
                };
            let carta25 = Carta {
                    nome: String::from("O Anel"),
                    numero: 25,
                    significado: String::from("Compromissos e contratos"),
                };
            let carta26 = Carta {
                    nome: String::from("O Livro"),
                    numero: 26,
                    significado: String::from("Segredos e conhecimento"),
                };
            let carta27 = Carta {
                    nome: String::from("A Carta"),
                    numero: 27,
                    significado: String::from("Mensagens e notícias"),
                };
            let carta28 = Carta {
                    nome: String::from("O Homem"),
                    numero: 28,
                    significado: String::from("Figura masculina ou o consulente"),
                };
            let carta29 = Carta {
                    nome: String::from("A Mulher"),
                    numero: 29,
                    significado: String::from("Figura feminina ou o consulente"),
                };
            let carta30 = Carta {
                    nome: String::from("O Lírio"),
                    numero: 30,
                    significado: String::from("Pureza e harmonia"),
                };
            let carta31 = Carta {
                    nome: String::from("O Sol"),
                    numero: 31,
                    significado: String::from("Sucesso e vitalidade"),
                };
            let carta32 = Carta {
                    nome: String::from("A Lua"),
                    numero: 32,
                    significado: String::from("Intuição e emoções ocultas"),
                };
            let carta33 = Carta {
                    nome: String::from("A Chave"),
                    numero: 33,
                    significado: String::from("Soluções e revelações"),
                };
            let carta34 = Carta {
                    nome: String::from("Os Peixes"),
                    numero: 34,
                    significado: String::from("Riqueza e abundância"),
                };
            let carta35 = Carta {
                    nome: String::from("A Âncora"),
                    numero: 35,
                    significado: String::from("Estabilidade e perseverança"),
                };
            let carta36 = Carta {
                    nome: String::from("A Cruz"),
                    numero: 36,
                    significado: String::from("Fardos e desafios espirituais"),
                }; 
                
                // 2. Coloque no vetor
                 let mut cartas_lenormand: Vec<Carta> = vec![carta1, carta2, carta3, carta4, carta5, carta6, carta7, carta8, carta9, carta10, carta11, carta12, carta13, carta14, carta15, carta16, carta17, carta18, carta19, carta20, carta21, carta22, carta23, carta24, carta25, carta26, carta27, carta28, carta29, carta30, carta31, carta32, carta33, carta34, carta35, carta36]; 

             // vec![...]: Cria uma lista (Vetor) com as cartas. 
             // mut: Importante! Se você não colocar mut, a lista fica "congelada" 
             // e você não consegue embaralhar nem tirar cartas de lá. 

        // 3. Embaralhe
        let mut rng = rand::rng(); 
        cartas_lenormand.shuffle(&mut rng);

        // 4. O Loop de tiragem
                println!("--- Mão Direita de Éris ---");
                for i in 0..5 { 
                if let Some(carta) = cartas_lenormand.pop() {
                    println!("{}: {} - {}", i + 1, carta.nome, carta.significado);
                } //fecha o if let
            } // fecha o for
        }, // fecha o EstiloTarot::Lenormand
       
//Aqui começa o código do Rider-Waite, eu coloquei as 22 cartas dos arcanos maiores.
        // Tiragem de Rider-Waite
        EstiloTarot::RiderWaite => {
            print!("Preparando sua tiragem de Rider-Waite... ");
            // 1. Crie as cartas aqui (r_carta1, r_carta2...) - Aqui você está usando o "molde" (struct) para criar um objeto real.
        //String::from(...): Em Rust, textos fixos são diferentes de Strings que podem ser manipuladas. Usamos o from para criar uma String real que o objeto Carta pode "possuir".
        // toda carta que eu adcionar aqui, eu tenho que colocar no vetor "r_cartaNumero" e no loop da tiragem. e depois no vetor "cartas_rider" para que ela apareça na tiragem.
            let r_carta1 = Carta {
                nome: String::from("O Louco"),
                numero: 0,
                significado: String::from("Novos começos"),
            };
            let r_carta2 = Carta {
                nome: String::from("A Sacerdotisa"),
                numero: 2,
                significado: String::from("Intuição e mistério"),
            };
            let r_carta3 = Carta {
                nome: String::from("O Mago"),
                numero: 1,
                significado: String::from("Poder e habilidade"),
            };
            let r_carta4 = Carta {
                nome: String::from("A Imperatriz"),
                numero: 3,
                significado: String::from("Fertilidade e criatividade"),
            };
            let r_carta5 = Carta {
                nome: String::from("O Imperador"),
                numero: 4,
                significado: String::from("Estrutura e autoridade"),
            };
            let r_carta6 = Carta {
                nome: String::from("O Hierofante"),
                numero: 5,
                significado: String::from("Tradição e espiritualidade"),
            };
            let r_carta7 = Carta {
                nome: String::from("Os Amantes"),
                numero: 6,
                significado: String::from("Amor e escolhas"),
            };
            let r_carta8 = Carta {
                nome: String::from("O Carro"),
                numero: 7,
                significado: String::from("Determinação e controle"),
            };
             let r_carta9 = Carta {
                nome: String::from("A Força"),
                numero: 8,
                significado: String::from("Coragem e compaixão"),
            };
             let r_carta10 = Carta {
                nome: String::from("O Eremita"),
                numero: 9,
                significado: String::from("Busca interior e sabedoria"),
            };
             let r_carta11 = Carta {
                nome: String::from("A Roda da Fortuna"),
                numero: 10,
                significado: String::from("Ciclos e mudanças"),
            };
             let r_carta12 = Carta {
                nome: String::from("A Justiça"),
                numero: 11,
                significado: String::from("Equilíbrio e verdade"),
            };
             let r_carta13 = Carta {
                nome: String::from("O Enforcado"),
                numero: 12,
                significado: String::from("Sacrifício e nova perspectiva"),
            };
             let r_carta14 = Carta {
                nome: String::from("A Morte"),
                numero: 13,
                significado: String::from("Transformação e renascimento"),
            };
             let r_carta15 = Carta {
                nome: String::from("A Temperança"),
                numero: 14,
                significado: String::from("Moderação e equilíbrio"),
            };
             let r_carta16 = Carta {
                nome: String::from("O Diabo"),
                numero: 15,
                significado: String::from("Vícios e materialismo"),
            };
             let r_carta17 = Carta {
                nome: String::from("A Torre"),
                numero: 16,
                significado: String::from("Ruína e revelação"),
            };
             let r_carta18 = Carta {
                nome: String::from("A Estrela"),
                numero: 17,
                significado: String::from("Esperança e inspiração"),
            };
             let r_carta19 = Carta {
                nome: String::from("A Lua"),
                numero: 18,
                significado: String::from("Ilusão e intuição"),
            };
             let r_carta20 = Carta {
                nome: String::from("O Sol"),
                numero: 19,
                significado: String::from("Sucesso e vitalidade"),
            };
             let r_carta21 = Carta {
                nome: String::from("O Julgamento"),
                numero: 20,
                significado: String::from("Renascimento e avaliação"),
            };
             let r_carta22 = Carta {
                nome: String::from("O Mundo"),
                numero: 21,
                significado: String::from("Realização e completude"),
            };


            // 2. Coloque no vetor
            let mut cartas_rider = vec![r_carta1, r_carta2, r_carta3, r_carta4, r_carta5, r_carta6, r_carta7, r_carta8, r_carta9, r_carta10, r_carta11, r_carta12, r_carta13, r_carta14, r_carta15, r_carta16, r_carta17, r_carta18, r_carta19, r_carta20, r_carta21, r_carta22]; // adicione as outras aqui
        //vec![...]: Cria uma lista (Vetor) com as cartas. -  mut: Importante! Se você não colocar mut, a lista fica "congelada" e você não consegue embaralhar nem tirar cartas de lá. 
       
            // 3. Embaralhe
        let mut rng = rand::rng(); //thread_rng() cria o motor de aleatoriedade.
            cartas_rider.shuffle(&mut rng);
        //.shuffle(&mut rng): É o comando que bagunça a ordem do vetor.    

            // 4. O Loop de tiragem
            println!("--- Mão Direita de Éris ---");
            for i in 0..5 { // mude para 5 se quiser 5 cartas
                if let Some(carta) = cartas_rider.pop() {
                    println!("{}: {} - {}", i + 1, carta.nome, carta.significado);
                }
            }
        }, // fecha o EstiloTarot::RiderWaite

//Aqui começa o código da Sibilla Italiana, eu coloquei 52 cartas.
        EstiloTarot::SibillaItaliana => {
            println!("Preparando sua tiragem de La Vera Sibilla...");

            // --- NAIPE DE COPAS (Sentimentos/Família) ---
            let s1 = Carta { nome: String::from("Il Gran Signore (O Grande Senhor)"), numero: 1, significado: String::from("Homem protetor, generoso, estabilidade.") };
            let s2 = Carta { nome: String::from("La Gran Signora (A Grande Senhora)"), numero: 2, significado: String::from("Mulher virtuosa, mãe, conselheira.") };
            let s3 = Carta { nome: String::from("L'Amante (O Amante)"), numero: 3, significado: String::from("Paixão, atração, o consulente masculino.") };
            let s4 = Carta { nome: String::from("L'Amante (A Amante)"), numero: 4, significado: String::from("Afeto, a consulente feminina.") };
            let s5 = Carta { nome: String::from("Il Pensiero (O Pensamento)"), numero: 5, significado: String::from("Reflexão, preocupação, planos.") };
            let s6 = Carta { nome: String::from("L'Amore (O Amor)"), numero: 6, significado: String::from("Felicidade, união, sentimentos profundos.") };
            let s7 = Carta { nome: String::from("L'Impeneo (O Casamento)"), numero: 7, significado: String::from("Contrato, compromisso, festa.") };
            let s8 = Carta { nome: String::from("La Casa (A Casa)"), numero: 8, significado: String::from("Ambiente doméstico, segurança, intimidade.") };
            let s9 = Carta { nome: String::from("La Reconoscenza (O Reconhecimento)"), numero: 9, significado: String::from("Gratidão, presente, mérito reconhecido.") };
            let s10 = Carta { nome: String::from("L'Attesa (A Espera)"), numero: 10, significado: String::from("Paciência, algo que demora a chegar.") };
            let s11 = Carta { nome: String::from("La Speranza (A Esperança)"), numero: 11, significado: String::from("Fé, sonhos, espera positiva.") };
            let s12 = Carta { nome: String::from("La Fedeltà (A Fidelidade)"), numero: 12, significado: String::from("Lealdade, constância, amigo fiel.") };
            let s13 = Carta { nome: String::from("La Costanza (A Constância)"), numero: 13, significado: String::from("Persistência, nada muda por enquanto.") };

            // --- NAIPE DE OUROS (Dinheiro/Negócios) ---
            let s14 = Carta { nome: String::from("Il Mercante (O Mercador)"), numero: 14, significado: String::from("Negócios, comércio, lucro.") };
            let s15 = Carta { nome: String::from("La Falsità (A Falsidade)"), numero: 15, significado: String::from("Engano, mentiras, traição próxima.") };
            let s16 = Carta { nome: String::from("Il Denaro (O Dinheiro)"), numero: 16, significado: String::from("Entrada de recursos, abundância.") };
            let s17 = Carta { nome: String::from("Deliranti (Delírios)"), numero: 17, significado: String::from("Confusão mental, excessos, ilusão.") };
            let s18 = Carta { nome: String::from("Il Ladro (O Ladrão)"), numero: 18, significado: String::from("Perda, roubo, alguém tirando sua energia.") };
            let s19 = Carta { nome: String::from("Messaggiere (O Mensageiro)"), numero: 19, significado: String::from("Notícias rápidas, telegrama, visita.") };
            let s20 = Carta { nome: String::from("Donna di Servizio (A Criada)"), numero: 10, significado: String::from("Ajuda, fofoca, alguém subalterno.") };
            let s21 = Carta { nome: String::from("I Denari (As Moedas)"), numero: 21, significado: String::from("Sucesso material, herança.") };
            let s22 = Carta { nome: String::from("La Lettera (A Carta)"), numero: 22, significado: String::from("Documento, notícia escrita, convite.") };
            let s23 = Carta { nome: String::from("La Sorpresa (A Surpresa)"), numero: 23, significado: String::from("Evento inesperado, sorte repentina.") };
            let s24 = Carta { nome: String::from("La Disperazione (O Desespero)"), numero: 24, significado: String::from("Crise financeira, perda de controle.") };
            let s25 = Carta { nome: String::from("L'Allegria (A Alegria)"), numero: 25, significado: String::from("Celebração, brinde, diversão.") };
            let s26 = Carta { nome: String::from("Il Viaggio (A Viagem)"), numero: 26, significado: String::from("Deslocamento, novos caminhos.") };

            // --- NAIPE DE PAUS (Trabalho/Ação) ---
            let s27 = Carta { nome: String::from("L'Imeneo (O Encontro)"), numero: 27, significado: String::from("Reunião, parceria de trabalho.") };
            let s28 = Carta { nome: String::from("La Superbia (A Vaidade)"), numero: 28, significado: String::from("Orgulho, arrogância, aparências.") };
            let s29 = Carta { nome: String::from("Il Successo (O Sucesso)"), numero: 29, significado: String::from("Vitória, objetivo alcançado.") };
            let s30 = Carta { nome: String::from("Il Malato (O Doente)"), numero: 30, significado: String::from("Estagnação, fraqueza, pausa forçada.") };
            let s31 = Carta { nome: String::from("La Morte (A Morte)"), numero: 31, significado: String::from("Corte radical, fim de ciclo.") };
            let s32 = Carta { nome: String::from("Il Sospiro (O Suspiro)"), numero: 32, significado: String::from("Desejo, saudade, algo distante.") };
            let s33 = Carta { nome: String::from("La Giovine Fanciulla (A Jovem)"), numero: 33, significado: String::from("Novidade, pureza, filha ou amiga.") };
            let s34 = Carta { nome: String::from("Il Dottore (O Médico)"), numero: 34, significado: String::from("Cura, conselho profissional, autoridade.") };
            let s35 = Carta { nome: String::from("La Conversazione (A Conversa)"), numero: 35, significado: String::from("Reunião social, papo importante.") };
            let s36 = Carta { nome: String::from("Il Medico (O Erudito)"), numero: 36, significado: String::from("Estudo, inteligência, sabedoria.") };
            let s37 = Carta { nome: String::from("La Fortuna (A Fortuna)"), numero: 37, significado: String::from("Sorte absoluta, destino favorável.") };
            let s38 = Carta { nome: String::from("La Gran Consolazione (O Grande Consolo)"), numero: 38, significado: String::from("Alívio, fim de um problema.") };
            let s39 = Carta { nome: String::from("La Malinconia (A Melancolia)"), numero: 39, significado: String::from("Tristeza, desânimo, passado.") };

            // --- NAIPE DE ESPADAS (Dificuldades/Mente) ---
            let s40 = Carta { nome: String::from("Il Militare (O Militar)"), numero: 40, significado: String::from("Disciplina, segredos, força.") };
            let s41 = Carta { nome: String::from("Il Nemico (O Inimigo)"), numero: 41, significado: String::from("Hostilidade, oposição direta.") };
            let s42 = Carta { nome: String::from("La Nemica (A Inimiga)"), numero: 42, significado: String::from("Mulher rival, fofoca maldosa.") };
            let s43 = Carta { nome: String::from("Il Sacerdote (O Sacerdote)"), numero: 43, significado: String::from("Lei, ética, proteção espiritual.") };
            let s44 = Carta { nome: String::from("La Prigione (A Prisão)"), numero: 44, significado: String::from("Bloqueio, solidão, isolamento.") };
            let s45 = Carta { nome: String::from("Gran Pena (Grande Pena)"), numero: 45, significado: String::from("Dor emocional profunda, luto.") };
            let s46 = Carta { nome: String::from("La Vedova (A Viúva)"), numero: 46, significado: String::from("Solidão, experiência através da dor.") };
            let s47 = Carta { nome: String::from("L'Allegrezza al Cuore (Alegria no Coração)"), numero: 47, significado: String::from("Superação de obstáculos.") };
            let s48 = Carta { nome: String::from("La Vecchia Signora (A Velha Senhora)"), numero: 48, significado: String::from("Sabedoria ancestral, passado.") };
            let s49 = Carta { nome: String::from("Il Delitto (O Delito)"), numero: 49, significado: String::from("Erro, crime, ação má pensada.") };
            let s50 = Carta { nome: String::from("La Disgrazia (A Desgraça)"), numero: 50, significado: String::from("Imprevisto negativo, choque.") };
            let s51 = Carta { nome: String::from("La Gelosia (A Inveja)"), numero: 51, significado: String::from("Ciumes, inveja, olhos em cima.") };
            let s52 = Carta { nome: String::from("L'Ufficiale (O Oficial)"), numero: 52, significado: String::from("Justiça sendo feita, clareza.") };

            // O VETOR (A parte que agrupa tudo)
            let mut cartas_sibilla = vec![
                s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15, s16, s17, s18, s19, s20,
                s21, s22, s23, s24, s25, s26, s27, s28, s29, s30, s31, s32, s33, s34, s35, s36, s37, s38, s39, s40,
                s41, s42, s43, s44, s45, s46, s47, s48, s49, s50, s51, s52];

            // EMBARALHAR E TIRAR
            let mut rng = rand::rng();
            cartas_sibilla.shuffle(&mut rng);

            println!("--- Tiragem La Vera Sibilla ---");
            for i in 0..5 {
                if let Some(carta) = cartas_sibilla.pop() {
                    println!("{}: {} - {}", i + 1, carta.nome, carta.significado);
                }
            }
        }, // fecha o braço da Sibilla
    } // fecha o match
} // fecha a fnmain