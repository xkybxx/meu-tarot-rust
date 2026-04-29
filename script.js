const btn = document.getElementById('btn-tirar-carta');
const divResultado = document.getElementById('resultado-oraculo');

btn.addEventListener('click', () => {
    // 1. Mostra que está carregando
    divResultado.innerText = "Consultando o destino...";

    // 2. Faz a chamada para o seu Rust
    fetch('http://localhost:8080/sua-rota') 
        .then(response => response.json()) // Converte a resposta em JSON
        .then(data => {
            // 3. Atualiza o HTML com o que veio do Rust
            divResultado.innerText = `Carta sorteada: ${data.nome} - ${data.significado}`;
        })
        .catch(error => {
            console.error('Erro:', error);
            divResultado.innerText = "Erro ao consultar o oráculo.";
        });
});
