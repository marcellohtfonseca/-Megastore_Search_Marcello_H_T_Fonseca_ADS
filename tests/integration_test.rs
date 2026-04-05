use megastore_search::{Catalogo, Produto};

#[test]
fn test_busca() {
    let mut catalogo = Catalogo::new();
    catalogo.adicionar_produto(Produto {
        id: 1,
        nome: "Geladeira Brastemp".to_string(),
        categoria: "Eletrodomésticos".to_string(),
    });

    let resultado = catalogo.buscar_por_nome("geladeira");
    assert_eq!(resultado.len(), 1);
}
