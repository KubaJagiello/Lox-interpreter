trait Visitor<T> { 
struct Binary {
left: Expr
operator: Token
right: Expr
}

struct Grouping {
expression: Expr
}

struct Literal {
value: Object
}

struct Unary {
operator: Token
right: Expr
}

}
