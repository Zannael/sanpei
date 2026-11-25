export type ResearchParameter = {
 id:string; label:string; kind:'number'|'boolean'|'category'; source:string;
 unit?:string|null; options?:string[]; minimum?:number|null; maximum?:number|null;
};
export type ResearchDiagram = {
 kind:string; main_cm?:number; terminal_cm?:number; float_g?:number;
 branches?:{anchor_cm:number;length_cm:number}[];
 weights?:{height_cm:number;mass_g:number;kind?:string}[];
};
