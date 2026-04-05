MegaStore – Sistema de Busca Avançado para Catálogo de Produtos
🧩 Visão Geral
O MegaStore é um sistema de busca desenvolvido em Rust para otimizar pesquisas em grandes catálogos de e-commerce.
Ele utiliza tabelas hash (HashMap) para indexar produtos de forma eficiente, garantindo consultas rápidas e resultados precisos mesmo em bases com milhões de itens.

🎯 Objetivos
Aumentar a velocidade e precisão das buscas.

Implementar uma estrutura de dados otimizada para indexação.

Garantir escalabilidade e alto desempenho.

Aplicar boas práticas de modularização e testes automatizados.

⚙️ Funcionalidades
Indexação por nome, categoria e marca.

Busca direta e filtrada com retorno instantâneo.

Armazenamento eficiente com HashMap.

Estrutura modular e expansível.

🚀 Tecnologias
Linguagem: Rust

Gerenciador de pacotes: Cargo

Estrutura de dados: HashMap

Testes automatizados: Cargo Test

Ambiente: Visual Studio Code

🧠 Estrutura do Projeto
MegaStore/
├── src/
│   ├── main.rs – Função principal e inicialização
│   ├── catalog.rs – Definição e indexação de produtos
│   ├── search.rs – Módulo de busca com HashMap
│   └── utils.rs – Funções auxiliares
├── tests/
│   ├── integration_test.rs
│   └── unit_test.rs
├── Cargo.toml – Configurações e dependências
└── README.md

🧮 Estruturas e Algoritmos
O núcleo do sistema utiliza uma tabela hash (HashMap) para armazenar e recuperar produtos de forma eficiente.
Cada produto é indexado por uma chave única, permitindo buscas em tempo constante (O(1)).

Exemplo de uso:

rust
use std::collections::HashMap;

fn main() {
    let mut catalogo = HashMap::new();
    catalogo.insert("Notebook Dell", "Eletrônicos");
    catalogo.insert("Camiseta Polo", "Vestuário");
}
🧪 Execução e Testes
Pré-requisitos
Instalar Rust e Cargo.

Executando o sistema
Clone o repositório e compile:

Código
git clone https://github.com/SeuUsuario/MegaStore.git
cd MegaStore
cargo build
cargo run
Testes
Os testes garantem o funcionamento dos módulos de busca e indexação:

Código
cargo test
Eles estão no diretório tests/ e incluem testes unitários e de integração.

💡 Exemplos de Uso
Busca por nome:

Código
Digite o nome do produto: notebook
Resultado: [ "Notebook Dell Inspiron 15", "Notebook HP Pavilion 14" ]
Busca por categoria:

Código
Buscar por categoria: Eletrônicos
Resultado: [ "Smartphone Samsung A55", "Notebook Dell Inspiron 15" ]
📈 Desempenho e Escalabilidade
O uso de HashMap reduz o tempo médio de busca para O(1), mesmo em grandes volumes de dados.
A arquitetura modular permite inserção dinâmica de novos produtos, adaptação para índices compostos e integração futura com bancos de dados ou APIs REST.

🤝 Contribuições
Contribuições são bem-vindas.
Para colaborar:

Faça um fork do projeto.

Crie uma branch: git checkout -b feature/minha-feature.

Envie um pull request descrevendo suas alterações.

📜 Licença
Este projeto está licenciado sob a MIT License — consulte o arquivo LICENSE para mais detalhes.

🔗 Repositório
Repositório disponível em:
https://github.com/marcellohtfonseca/-Megastore_Search_Marcello_H_T_Fonseca_ADS
