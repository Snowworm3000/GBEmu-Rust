struct Registers{
    A: i8,
    B: i8,
    C: i8,
    D: i8,
    E: i8,
    H: i8,
    L: i8,

}

struct CPU{

}

enum Flags{
    Z = (1 << 7),
    N = (1 << 6),
    H = (1 << 5),
    C = (1 << 4),
}