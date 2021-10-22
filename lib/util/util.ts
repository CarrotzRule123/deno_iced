import { Elements, ElementType } from "../element.ts";

export function ElementFromName(name: Elements): ElementType {
    return {
        "button": ElementType.Button,
        "text": ElementType.Text,
    }[name]
}