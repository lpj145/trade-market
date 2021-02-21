### Produto
Isto é um produto minímo viável pensando tarde da noite de como possivelmente pode funcionar um mercado de negócios, onde se compra e vende ações, bem como se participa das suas flutuações, hehe desenvolver isso em quanto aprendo Rust.

### Porque ?

Quando aprendemos novas linguagens de programação é interessante para que elas façam sentido na nossa mente: testar conceitos, explorar os limites do obvio,
tentar explodir se possível o compilador.

 Isso nos dará uma maior propriedade pra resolver problemas, a medida que evoluímos a gente também consegue medir se: a espada que a gente afia não ta desproporcional ao gigante que a gente quer matar, afinal porque entrar numa luta que não da para ganhar ? Isso só nos levará a frustação e obviamente a desistência. Hehehe assim nasceu esse projeto, veio simples, complicou e complicou, mas deve sair, em breve.

### Estória
Sem adentrar nos mínimos detalhes se não nunca vou terminar, esse projeto deve funcionar como um lugar onde teremos um **ARMAZEM** de **AÇÕES**, colocaremos **PREÇO** e **NOME** em cada uma delas, teremos canais de interação com usuários **COMPRADORES** que darão seus **LANCES** se pro canal fizer sentido, então ele vende e gera **TENDÊNCIAS** na **AÇÃO**, o mesmo deve acontecer na **COMPRA**.

Para equalizar as coisas e melhorar o sentido, a partir daqui chamaremos:

 - ARMAZEM == BROKER
 - COMPRADOR/VENDEDOR == INTERESSADO 
 - COMPRAR/VENDER == INTENÇÃO

### Como as coisas devem funcionar:

#### BROKER
- [X] REGISTRAR interessados em participar do mercado.
- [X] REGISTRAR ações, ou colocar no "deposito" cada uma delas caracterizada por: nome, valor e quantidade.
- [X] REGISTRAR intenções dos interessados.
- [X] REGISTRAR canais de interação com ações que são limitados adois tipos: COMPRA e VENDA.
- [X] GERAR número incremental da intenção, para que todas elas tenham ordem de interesse (quem fez primeiro).
- [ ] EXECUTAR intenções quando as condições fizerem sentido.
- [ ] Mostrar pro público a consolidação do resultado das intenções olhando para ações, quanto tem em ações guardado, qual valor.
- [ ] Mostrar pro público a consolidação da realização das intenções dos interessados, quanto ele tem, quais ações tem e quanto tem em dinheiro.

#### INTERESSADO
- [X] REGISTRAR na carteira o dinheiro que possuí disponivel.
- [ ] REGISTRAR por meio de um canal de interesse uma intenção.

### Cenário final:
Quando o cli estiver pronto, ele deve ser capaz de gerar interessados, ações e executar as intenções sozinho, e mostrando o resultado após alguma determinada condição que irei pensar.