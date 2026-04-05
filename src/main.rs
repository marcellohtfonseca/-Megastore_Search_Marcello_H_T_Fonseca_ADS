use megastore_search::{Catalogo, Produto};

fn main() {
    let mut catalogo = Catalogo::new();

    catalogo.adicionar_produto(Produto {
        id: 1,
        nome: "Smartphone Samsung".to_string(),
        categoria: "Eletrônicos".to_string(),
    });

    catalogo.adicionar_produto(Produto {
        id: 2,
        nome: "Camiseta Nike".to_string(),
        categoria: "Vestuário".to_string(),
    });

    let resultados = catalogo.buscar_por_nome("samsung");
    for produto in resultados {
        println!("{:?}", produto);
    }
}
