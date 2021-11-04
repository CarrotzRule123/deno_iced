import { library } from "../main.ts";
import { State } from "./state.ts";

export type Elements =
    | "button"
    | "text"

export enum ElementType {
    Button,
    Text
}

export class Element {
    private _created = false
    constructor(
        private rid: number,
        private type: ElementType
    ) { }

    public set(state: State, value: any) {
        // library.symbols.ops_add_child_element()
    }

    public addElement(element: Element) {
        // library.symbols.ops_add_child_element(this.rid, element.type)
    }
}

