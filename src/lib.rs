use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
}

pub struct Catalogo {
    produtos: HashMap<u32, Produto>,
}

impl Catalogo {
    pub fn new() -> Self {
        Catalogo {
            produtos: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        self.produtos.insert(produto.id, produto);
    }

    pub fn buscar_por_nome(&self, termo: &str) -> Vec<&Produto> {
        self.produtos
            .values()
            .filter(|p| p.nome.to_lowercase().contains(&termo.to_lowercase()))
            .collect()
    }
}
