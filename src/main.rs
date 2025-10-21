mod cell;
mod lazycell;
mod oncecell;
mod refcell;
mod reference;
mod smartpointer;
fn main() {
    reference::reference_operation();
    smartpointer::smartpointers_opaeration();
    refcell::refoperation();
    oncecell::oncecell();
    lazycell::lazy_operation();
    cell::cell_operation();
}
