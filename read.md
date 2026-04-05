# 🛍️ Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 🧩 Descrição do Projeto
O **Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore** tem como objetivo aprimorar a experiência de busca em grandes catálogos de e-commerce.  
Desenvolvido em **Rust**, o sistema utiliza **tabelas hash (HashMap)** para indexar produtos de forma eficiente, garantindo **rápido acesso** às informações e **consultas precisas**, mesmo em bases de dados extensas.

### 🎯 Objetivos
- Melhorar a velocidade e precisão das buscas no catálogo da MegaStore.  
- Implementar uma estrutura de dados eficiente para indexação de produtos.  
- Garantir escalabilidade e desempenho diante de milhões de itens.  
- Aplicar boas práticas de modularização e testes automatizados em Rust.

### ⚙️ Funcionalidades Principais
- Indexação de produtos por **nome**, **categoria** e **marca**.  
- Busca direta e filtrada com retorno rápido.  
- Armazenamento otimizado com uso de **HashMap**.  
- Estrutura modular, permitindo fácil manutenção e expansão.  

---

## 🚀 Tecnologias Utilizadas
- **Linguagem:** Rust  
- **Gerenciador de Pacotes:** Cargo  
- **Estrutura de Dados:** HashMap (tabela hash)  
- **Ferramentas de Teste:** Cargo Test  
- **Ambiente de Desenvolvimento:** Visual Studio Code  

---

## 🧠 Arquitetura do Sistema
O sistema foi desenvolvido com foco em **modularidade e eficiência**, utilizando a seguinte estrutura de diretórios:

MegaStore/
├── src/
│ ├── main.rs # Função principal e inicialização do sistema
│ ├── catalog.rs # Módulo de definição e indexação de produtos
│ ├── search.rs # Módulo de busca com uso de HashMap
│ └── utils.rs # Funções auxiliares
├── tests/
│ ├── integration_test.rs
│ └── unit_test.rs
├── Cargo.toml # Configurações e dependências do projeto
└── README.md


---

## 🧮 Algoritmos e Estruturas de Dados Utilizados
O núcleo da aplicação baseia-se em uma **tabela hash (`HashMap`)** para armazenar e recuperar produtos de maneira eficiente.  
Cada produto é indexado por uma chave única (ex.: nome do produto), permitindo buscas em **tempo constante (O(1))** na maioria dos casos.

Exemplo simplificado:
```rust
use std::collections::HashMap;

fn main() {
    let mut catalogo = HashMap::new();
    catalogo.insert("Notebook Dell", "Eletrônicos");
    catalogo.insert("Camiseta Polo", "Vestuário");
}

🧪 Instruções de Execução
🔧 Pré-requisitos

Ter o Rust e o Cargo instalados.
Instalar Rust

▶️ Executando o Sistema

Clone o repositório do projeto:

git clone https://github.com/SeuUsuario/MegaStore.git
cd MegaStore


Compile o projeto:

cargo build


Execute o sistema:

cargo run

🧩 Instruções de Teste

Os testes foram desenvolvidos para validar o funcionamento dos módulos de busca e indexação.

Para executar todos os testes:

cargo test


Os testes estão localizados no diretório tests/ e incluem:

Testes unitários: validações de funções individuais.

Testes de integração: validações do sistema completo em cenários reais.

💡 Exemplos de Uso

Após a execução, o sistema permite realizar buscas por produtos no catálogo.
Exemplo:

> Digite o nome do produto: notebook
Resultado: [ "Notebook Dell Inspiron 15", "Notebook HP Pavilion 14" ]


Busca filtrada:

> Buscar por categoria: Eletrônicos
Resultado: [ "Smartphone Samsung A55", "Notebook Dell Inspiron 15" ]

📈 Desempenho e Escalabilidade

O uso da estrutura HashMap reduz o tempo médio de busca para O(1), mesmo com grandes volumes de dados.
A arquitetura modular permite:

Inserção dinâmica de novos produtos.

Adaptação futura para buscas com índices compostos (por nome e categoria).

Integração com bancos de dados ou APIs REST para escalabilidade.

🤝 Contribuições

Contribuições são bem-vindas!
Para colaborar:

Faça um fork do projeto.

Crie uma nova branch (git checkout -b feature/minha-feature).

Envie um pull request descrevendo suas alterações.

📜 Licença

Este projeto está licenciado sob a MIT License — veja o arquivo LICENSE
 para mais detalhes.

🔗 Repositório

Repositório disponível em:
👉 [https://github.com/SeuUsuario/MegaStore](https://github.com/AleffBlendon/Megastore_Search_Aleff_Blendon_Costa_ADS_2EC)
