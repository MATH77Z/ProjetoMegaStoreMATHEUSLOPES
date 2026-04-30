use std::collections::HashMap;

#[derive(Debug)]
struct Produto {
    nome: String,
    categoria: String,
    marca: String,
    preco: f64,
}

fn main() {

    let mut catalogo: HashMap<String, Produto> = HashMap::new();

    catalogo.insert(
        "notebook".to_string(),
        Produto {
            nome: "Notebook Gamer".to_string(),
            categoria: "Eletrônicos".to_string(),
            marca: "Dell".to_string(),
            preco: 4500.0,
        },
    );

    catalogo.insert(
        "mouse".to_string(),
        Produto {
            nome: "Mouse Gamer".to_string(),
            categoria: "Periféricos".to_string(),
            marca: "Logitech".to_string(),
            preco: 150.0,
        },
    );

    let busca = "notebook";

    match catalogo.get(busca) {
        Some(produto) => {
            println!("Produto encontrado:");
            println!("Nome: {}", produto.nome);
            println!("Categoria: {}", produto.categoria);
            println!("Marca: {}", produto.marca);
            println!("Preço: R${}", produto.preco);
        }

        None => {
            println!("Produto não encontrado");
        }
    }
}