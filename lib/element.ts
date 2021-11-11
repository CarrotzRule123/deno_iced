import { library } from "./bindings.ts";
import { State } from "./state.ts";
import { ElementId } from "./util/util.ts";

export type Elements =
    | "button"
    | "text"
    | "div"

export enum ElementType {
    Button,
    Text,
    Div
}

export class Element {
    public _created = false
    public _rid: number
    public _type: ElementType
    constructor(type: ElementType) { 
        this._type = type
        this._rid = ElementId()
    }

    public set(state: State, value: any) {
        // library.symbols.ops_add_child_element()
    }

    public appendChild(element: Element) {
        library.symbols.ops_add_child_element(element._type, this._rid, element._rid)
    }
}

