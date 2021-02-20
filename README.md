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
- [ ] REGISTRAR interessados em participar do mercado.
- [ ] REGISTRAR ações, ou colocar no "deposito" cada uma delas caracterizada por: nome, valor e quantidade.
- [ ] REGISTRAR intenções dos interessados.
- [ ] REGISTRAR canais de interação com ações que são limitados adois tipos: COMPRA e VENDA.
- [ ] GERAR número incremental da intenção, para que todas elastenham ordem de interesse (quem fez primeiro).
- [ ] EXECUTAR inteções quando as condições fizerem sentido.
- [ ] Mostrar pro público a consolidação do resultado das intençõesolhando para ações, quanto tem em ações guardado, qual valor.
- [ ] Mostrar pro público a consolidação da realização das intençõesdos interessados, quanto ele tem, quais ações tem e quanto tem emdinheiro.

#### INTERESSADO
- [ ] REGISTRAR na carteira o dinheiro que possuí disponivel.
- [ ] REGISTRAR por meio de um canal de interesse uma intenção.