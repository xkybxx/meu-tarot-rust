use axum::{
    extract::Query,
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

// ─── Estruturas ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
struct Carta {
    nome: String,
    numero: u8,
    significado: String,
}

#[derive(Debug, Clone, Serialize)]
struct CartaNaPosicao {
    posicao: String,
    carta: Carta,
}

#[derive(Deserialize)]
struct TiragemParams {
    deck: Option<String>,
    tiragem: Option<String>,
}

#[derive(Serialize)]
struct TiragemResponse {
    deck: String,
    tiragem: String,
    cartas: Vec<CartaNaPosicao>,
}

#[derive(Serialize)]
struct SimNaoResponse {
    deck: String,
    tiragem: String,
    carta: CartaNaPosicao,
    resposta: String,
    explicacao: String,
}

#[derive(Serialize)]
struct ErroResponse {
    erro: String,
    decks_disponiveis: Vec<String>,
    tiragens_disponiveis: Vec<String>,
}

// ─── Posições das Tiragens ────────────────────────────────────────────────────

fn posicoes_passado_presente_futuro() -> Vec<&'static str> {
    vec!["Passado", "Presente", "Futuro"]
}

fn posicoes_templo_afrodite() -> Vec<&'static str> {
    vec![
        "O seu coração",
        "O seu pensamento",
        "A sua chama",
        "O coração do outro",
        "O pensamento do outro",
        "A chama do outro",
        "O futuro — O conselho de Afrodite",
    ]
}

fn posicoes_eris() -> Vec<&'static str> {
    vec![
        "A pergunta",
        "O que pode atrapalhar",
        "O que pode ajudar",
        "Possível resultado 1",
        "Possível resultado 2",
    ]
}

fn posicao_sim_nao() -> &'static str {
    "A resposta do cosmos"
}

// ─── Lógica Sim ou Não ────────────────────────────────────────────────────────

fn interpretar_sim_nao(numero: u8, nome: &str) -> (&'static str, &'static str) {
    // Cartas de energia positiva = Sim, negativa = Não, neutras = Talvez
    match numero % 3 {
        0 => ("✨ Sim", "As energias se alinham a seu favor. O cosmos aponta para frente."),
        1 => ("🌑 Não", "As estrelas pedem cautela. Este não é o momento certo."),
        _ => ("🌗 Talvez", "O véu ainda está espesso. Medite sobre a questão antes de agir."),
    }
}

// ─── Decks ────────────────────────────────────────────────────────────────────

fn deck_lenormand() -> Vec<Carta> {
    vec![
        Carta { nome: "O Cavaleiro".into(),   numero: 1,  significado: "Notícias e movimento".into() },
        Carta { nome: "O Trevo".into(),        numero: 2,  significado: "Sorte e oportunidades".into() },
        Carta { nome: "O Navio".into(),        numero: 3,  significado: "Viagens e mudanças".into() },
        Carta { nome: "A Casa".into(),         numero: 4,  significado: "Lar e família".into() },
        Carta { nome: "A Árvore".into(),       numero: 5,  significado: "Saúde e crescimento".into() },
        Carta { nome: "As Nuvens".into(),      numero: 6,  significado: "Confusão e incerteza".into() },
        Carta { nome: "A Serpente".into(),     numero: 7,  significado: "Engano e traição".into() },
        Carta { nome: "O Caixão".into(),       numero: 8,  significado: "Fim de ciclo e transformação".into() },
        Carta { nome: "O Buquê".into(),        numero: 9,  significado: "Felicidade e celebração".into() },
        Carta { nome: "A Foice".into(),        numero: 10, significado: "Cortes e decisões difíceis".into() },
        Carta { nome: "O Chicote".into(),      numero: 11, significado: "Conflitos e discussões".into() },
        Carta { nome: "Os Pássaros".into(),    numero: 12, significado: "Comunicação e ansiedade".into() },
        Carta { nome: "A Criança".into(),      numero: 13, significado: "Inocência e novos começos".into() },
        Carta { nome: "A Raposa".into(),       numero: 14, significado: "Astúcia e trabalho".into() },
        Carta { nome: "O Urso".into(),         numero: 15, significado: "Força e proteção".into() },
        Carta { nome: "A Estrela".into(),      numero: 16, significado: "Esperança e inspiração".into() },
        Carta { nome: "A Cegonha".into(),      numero: 17, significado: "Mudanças e novidades".into() },
        Carta { nome: "O Cão".into(),          numero: 18, significado: "Amizade e lealdade".into() },
        Carta { nome: "A Torre".into(),        numero: 19, significado: "Isolamento e autoridade".into() },
        Carta { nome: "O Jardim".into(),       numero: 20, significado: "Socialização e eventos".into() },
        Carta { nome: "A Montanha".into(),     numero: 21, significado: "Desafios e obstáculos".into() },
        Carta { nome: "O Caminho".into(),      numero: 22, significado: "Decisões e escolhas".into() },
        Carta { nome: "Os Ratos".into(),       numero: 23, significado: "Perdas e preocupações".into() },
        Carta { nome: "O Coração".into(),      numero: 24, significado: "Amor e emoções".into() },
        Carta { nome: "O Anel".into(),         numero: 25, significado: "Compromissos e contratos".into() },
        Carta { nome: "O Livro".into(),        numero: 26, significado: "Segredos e conhecimento".into() },
        Carta { nome: "A Carta".into(),        numero: 27, significado: "Mensagens e notícias".into() },
        Carta { nome: "O Homem".into(),        numero: 28, significado: "Figura masculina ou o consulente".into() },
        Carta { nome: "A Mulher".into(),       numero: 29, significado: "Figura feminina ou o consulente".into() },
        Carta { nome: "O Lírio".into(),        numero: 30, significado: "Pureza e harmonia".into() },
        Carta { nome: "O Sol".into(),          numero: 31, significado: "Sucesso e vitalidade".into() },
        Carta { nome: "A Lua".into(),          numero: 32, significado: "Intuição e emoções ocultas".into() },
        Carta { nome: "A Chave".into(),        numero: 33, significado: "Soluções e revelações".into() },
        Carta { nome: "Os Peixes".into(),      numero: 34, significado: "Riqueza e abundância".into() },
        Carta { nome: "A Âncora".into(),       numero: 35, significado: "Estabilidade e perseverança".into() },
        Carta { nome: "A Cruz".into(),         numero: 36, significado: "Fardos e desafios espirituais".into() },
    ]
}

fn deck_rider_waite() -> Vec<Carta> {
    vec![
        Carta { nome: "O Louco".into(),           numero: 0,  significado: "Novos começos".into() },
        Carta { nome: "O Mago".into(),             numero: 1,  significado: "Poder e habilidade".into() },
        Carta { nome: "A Sacerdotisa".into(),      numero: 2,  significado: "Intuição e mistério".into() },
        Carta { nome: "A Imperatriz".into(),       numero: 3,  significado: "Fertilidade e criatividade".into() },
        Carta { nome: "O Imperador".into(),        numero: 4,  significado: "Estrutura e autoridade".into() },
        Carta { nome: "O Hierofante".into(),       numero: 5,  significado: "Tradição e espiritualidade".into() },
        Carta { nome: "Os Amantes".into(),         numero: 6,  significado: "Amor e escolhas".into() },
        Carta { nome: "O Carro".into(),            numero: 7,  significado: "Determinação e controle".into() },
        Carta { nome: "A Força".into(),            numero: 8,  significado: "Coragem e compaixão".into() },
        Carta { nome: "O Eremita".into(),          numero: 9,  significado: "Busca interior e sabedoria".into() },
        Carta { nome: "A Roda da Fortuna".into(),  numero: 10, significado: "Ciclos e mudanças".into() },
        Carta { nome: "A Justiça".into(),          numero: 11, significado: "Equilíbrio e verdade".into() },
        Carta { nome: "O Enforcado".into(),        numero: 12, significado: "Sacrifício e nova perspectiva".into() },
        Carta { nome: "A Morte".into(),            numero: 13, significado: "Transformação e renascimento".into() },
        Carta { nome: "A Temperança".into(),       numero: 14, significado: "Moderação e equilíbrio".into() },
        Carta { nome: "O Diabo".into(),            numero: 15, significado: "Vícios e materialismo".into() },
        Carta { nome: "A Torre".into(),            numero: 16, significado: "Ruína e revelação".into() },
        Carta { nome: "A Estrela".into(),          numero: 17, significado: "Esperança e inspiração".into() },
        Carta { nome: "A Lua".into(),              numero: 18, significado: "Ilusão e intuição".into() },
        Carta { nome: "O Sol".into(),              numero: 19, significado: "Sucesso e vitalidade".into() },
        Carta { nome: "O Julgamento".into(),       numero: 20, significado: "Renascimento e avaliação".into() },
        Carta { nome: "O Mundo".into(),            numero: 21, significado: "Realização e completude".into() },
    ]
}

fn deck_sibilla() -> Vec<Carta> {
    vec![
        Carta { nome: "Il Gran Signore (O Grande Senhor)".into(),          numero: 1,  significado: "Homem protetor, generoso, estabilidade.".into() },
        Carta { nome: "La Gran Signora (A Grande Senhora)".into(),         numero: 2,  significado: "Mulher virtuosa, mãe, conselheira.".into() },
        Carta { nome: "L'Amante (O Amante)".into(),                        numero: 3,  significado: "Paixão, atração, o consulente masculino.".into() },
        Carta { nome: "L'Amante (A Amante)".into(),                        numero: 4,  significado: "Afeto, a consulente feminina.".into() },
        Carta { nome: "Il Pensiero (O Pensamento)".into(),                 numero: 5,  significado: "Reflexão, preocupação, planos.".into() },
        Carta { nome: "L'Amore (O Amor)".into(),                           numero: 6,  significado: "Felicidade, união, sentimentos profundos.".into() },
        Carta { nome: "L'Impeneo (O Casamento)".into(),                    numero: 7,  significado: "Contrato, compromisso, festa.".into() },
        Carta { nome: "La Casa (A Casa)".into(),                           numero: 8,  significado: "Ambiente doméstico, segurança, intimidade.".into() },
        Carta { nome: "La Reconoscenza (O Reconhecimento)".into(),         numero: 9,  significado: "Gratidão, presente, mérito reconhecido.".into() },
        Carta { nome: "L'Attesa (A Espera)".into(),                        numero: 10, significado: "Paciência, algo que demora a chegar.".into() },
        Carta { nome: "La Speranza (A Esperança)".into(),                  numero: 11, significado: "Fé, sonhos, espera positiva.".into() },
        Carta { nome: "La Fedeltà (A Fidelidade)".into(),                  numero: 12, significado: "Lealdade, constância, amigo fiel.".into() },
        Carta { nome: "La Costanza (A Constância)".into(),                 numero: 13, significado: "Persistência, nada muda por enquanto.".into() },
        Carta { nome: "Il Mercante (O Mercador)".into(),                   numero: 14, significado: "Negócios, comércio, lucro.".into() },
        Carta { nome: "La Falsità (A Falsidade)".into(),                   numero: 15, significado: "Engano, mentiras, traição próxima.".into() },
        Carta { nome: "Il Denaro (O Dinheiro)".into(),                     numero: 16, significado: "Entrada de recursos, abundância.".into() },
        Carta { nome: "Deliranti (Delírios)".into(),                       numero: 17, significado: "Confusão mental, excessos, ilusão.".into() },
        Carta { nome: "Il Ladro (O Ladrão)".into(),                        numero: 18, significado: "Perda, roubo, alguém tirando sua energia.".into() },
        Carta { nome: "Messaggiere (O Mensageiro)".into(),                 numero: 19, significado: "Notícias rápidas, telegrama, visita.".into() },
        Carta { nome: "Donna di Servizio (A Criada)".into(),               numero: 20, significado: "Ajuda, fofoca, alguém subalterno.".into() },
        Carta { nome: "I Denari (As Moedas)".into(),                       numero: 21, significado: "Sucesso material, herança.".into() },
        Carta { nome: "La Lettera (A Carta)".into(),                       numero: 22, significado: "Documento, notícia escrita, convite.".into() },
        Carta { nome: "La Sorpresa (A Surpresa)".into(),                   numero: 23, significado: "Evento inesperado, sorte repentina.".into() },
        Carta { nome: "La Disperazione (O Desespero)".into(),              numero: 24, significado: "Crise financeira, perda de controle.".into() },
        Carta { nome: "L'Allegria (A Alegria)".into(),                     numero: 25, significado: "Celebração, brinde, diversão.".into() },
        Carta { nome: "Il Viaggio (A Viagem)".into(),                      numero: 26, significado: "Deslocamento, novos caminhos.".into() },
        Carta { nome: "L'Imeneo (O Encontro)".into(),                      numero: 27, significado: "Reunião, parceria de trabalho.".into() },
        Carta { nome: "La Superbia (A Vaidade)".into(),                    numero: 28, significado: "Orgulho, arrogância, aparências.".into() },
        Carta { nome: "Il Successo (O Sucesso)".into(),                    numero: 29, significado: "Vitória, objetivo alcançado.".into() },
        Carta { nome: "Il Malato (O Doente)".into(),                       numero: 30, significado: "Estagnação, fraqueza, pausa forçada.".into() },
        Carta { nome: "La Morte (A Morte)".into(),                         numero: 31, significado: "Corte radical, fim de ciclo.".into() },
        Carta { nome: "Il Sospiro (O Suspiro)".into(),                     numero: 32, significado: "Desejo, saudade, algo distante.".into() },
        Carta { nome: "La Giovine Fanciulla (A Jovem)".into(),             numero: 33, significado: "Novidade, pureza, filha ou amiga.".into() },
        Carta { nome: "Il Dottore (O Médico)".into(),                      numero: 34, significado: "Cura, conselho profissional, autoridade.".into() },
        Carta { nome: "La Conversazione (A Conversa)".into(),              numero: 35, significado: "Reunião social, papo importante.".into() },
        Carta { nome: "Il Medico (O Erudito)".into(),                      numero: 36, significado: "Estudo, inteligência, sabedoria.".into() },
        Carta { nome: "La Fortuna (A Fortuna)".into(),                     numero: 37, significado: "Sorte absoluta, destino favorável.".into() },
        Carta { nome: "La Gran Consolazione (O Grande Consolo)".into(),    numero: 38, significado: "Alívio, fim de um problema.".into() },
        Carta { nome: "La Malinconia (A Melancolia)".into(),               numero: 39, significado: "Tristeza, desânimo, passado.".into() },
        Carta { nome: "Il Militare (O Militar)".into(),                    numero: 40, significado: "Disciplina, segredos, força.".into() },
        Carta { nome: "Il Nemico (O Inimigo)".into(),                      numero: 41, significado: "Hostilidade, oposição direta.".into() },
        Carta { nome: "La Nemica (A Inimiga)".into(),                      numero: 42, significado: "Mulher rival, fofoca maldosa.".into() },
        Carta { nome: "Il Sacerdote (O Sacerdote)".into(),                 numero: 43, significado: "Lei, ética, proteção espiritual.".into() },
        Carta { nome: "La Prigione (A Prisão)".into(),                     numero: 44, significado: "Bloqueio, solidão, isolamento.".into() },
        Carta { nome: "Gran Pena (Grande Pena)".into(),                    numero: 45, significado: "Dor emocional profunda, luto.".into() },
        Carta { nome: "La Vedova (A Viúva)".into(),                        numero: 46, significado: "Solidão, experiência através da dor.".into() },
        Carta { nome: "L'Allegrezza al Cuore (Alegria no Coração)".into(), numero: 47, significado: "Superação de obstáculos.".into() },
        Carta { nome: "La Vecchia Signora (A Velha Senhora)".into(),       numero: 48, significado: "Sabedoria ancestral, passado.".into() },
        Carta { nome: "Il Delitto (O Delito)".into(),                      numero: 49, significado: "Erro, crime, ação má pensada.".into() },
        Carta { nome: "La Disgrazia (A Desgraça)".into(),                  numero: 50, significado: "Imprevisto negativo, choque.".into() },
        Carta { nome: "La Gelosia (A Inveja)".into(),                      numero: 51, significado: "Ciúmes, inveja, olhos em cima.".into() },
        Carta { nome: "L'Ufficiale (O Oficial)".into(),                    numero: 52, significado: "Justiça sendo feita, clareza.".into() },
    ]
}

// ─── Handler ──────────────────────────────────────────────────────────────────

async fn sortear(Query(params): Query<TiragemParams>) -> Response {
    let deck_nome = params.deck.unwrap_or_else(|| "rider".to_string());
    let tiragem_nome = params.tiragem.unwrap_or_else(|| "eris".to_string());

    let mut cartas = match deck_nome.to_lowercase().as_str() {
        "lenormand"                          => deck_lenormand(),
        "rider" | "riderwaite" | "rider-waite" => deck_rider_waite(),
        "sibilla"                            => deck_sibilla(),
        _ => {
            let erro = ErroResponse {
                erro: format!("Deck '{}' não encontrado.", deck_nome),
                decks_disponiveis: vec!["rider".into(), "lenormand".into(), "sibilla".into()],
                tiragens_disponiveis: vec!["eris".into(), "passado-presente-futuro".into(), "afrodite".into(), "sim-nao".into()],
            };
            return (StatusCode::BAD_REQUEST, Json(erro)).into_response();
        }
    };

    let mut rng = rand::rng();
    cartas.shuffle(&mut rng);

    // Tiragem Sim ou Não é especial — retorna estrutura diferente
    if tiragem_nome.to_lowercase() == "sim-nao" {
        let carta = cartas.into_iter().next().unwrap();
        let (resposta, explicacao) = interpretar_sim_nao(carta.numero, &carta.nome);
        let resultado = SimNaoResponse {
            deck: deck_nome,
            tiragem: "sim-nao".into(),
            carta: CartaNaPosicao {
                posicao: posicao_sim_nao().to_string(),
                carta,
            },
            resposta: resposta.to_string(),
            explicacao: explicacao.to_string(),
        };
        return Json(resultado).into_response();
    }

    let posicoes: Vec<&str> = match tiragem_nome.to_lowercase().as_str() {
        "passado-presente-futuro" | "ppf" => posicoes_passado_presente_futuro(),
        "afrodite" | "templo-afrodite"    => posicoes_templo_afrodite(),
        "eris" | "mao-de-eris"            => posicoes_eris(),
        _ => {
            let erro = ErroResponse {
                erro: format!("Tiragem '{}' não encontrada.", tiragem_nome),
                decks_disponiveis: vec!["rider".into(), "lenormand".into(), "sibilla".into()],
                tiragens_disponiveis: vec!["eris".into(), "passado-presente-futuro".into(), "afrodite".into(), "sim-nao".into()],
            };
            return (StatusCode::BAD_REQUEST, Json(erro)).into_response();
        }
    };

    let resultado: Vec<CartaNaPosicao> = posicoes
        .iter()
        .zip(cartas.into_iter())
        .map(|(posicao, carta)| CartaNaPosicao {
            posicao: posicao.to_string(),
            carta,
        })
        .collect();

    let resposta = TiragemResponse {
        deck: deck_nome,
        tiragem: tiragem_nome,
        cartas: resultado,
    };

    Json(resposta).into_response()
}

// ─── Servir arquivos estáticos ────────────────────────────────────────────────

async fn serve_index() -> impl IntoResponse {
    let html = include_str!("../index.html");
    axum::response::Html(html)
}

async fn serve_css() -> impl IntoResponse {
    let css = include_str!("../style.css");
    (
        [(axum::http::header::CONTENT_TYPE, "text/css")],
        css,
    )
}

// ─── Main ─────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/style.css", get(serve_css))
        .route("/sortear", get(sortear))
        .layer(cors);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("🔮 Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}