<script lang="ts">
 import type {ResearchDiagram} from './research-types';
 export let diagram:ResearchDiagram;
 $: weights=diagram.weights || [];
 $: maxHeight=Math.max(1,...weights.map(w=>w.height_cm));
 const fmt=(n:number)=>new Intl.NumberFormat('it-IT',{maximumFractionDigits:2}).format(n);
 $: total=weights.reduce((sum,w)=>sum+w.mass_g,0);
</script>
<figure class="rig-diagram">
<svg viewBox="0 0 380 290" role="img" aria-label="Schema della montatura, non in scala">
 <text x="20" y="22" class="label">Verso la canna</text>
 {#if diagram.kind==='bottom' && diagram.main_cm}
  <line x1="90" y1="35" x2="90" y2="224" class="main-line"/><circle cx="90" cy="35" r="3" class="main-fill"/>
  <text x="16" y="140" class="label">Trave</text><text x="16" y="157" class="label">{diagram.main_cm} cm</text>
  <rect x="69" y="224" width="42" height="22" rx="3" class="weight"/><text x="75" y="239" class="small">piombo</text>
  {#each diagram.branches || [] as branch}
   {@const y=224-(branch.anchor_cm/diagram.main_cm)*180}
   <circle cx="90" cy={y} r="5" class="junction"/><line x1="96" y1={y} x2="250" y2={y+7} class="branch-line"/><circle cx="250" cy={y+7} r="2" class="branch-fill"/>
   <text x="104" y={y-9} class="small">Snodo: {branch.anchor_cm} cm dal piombo</text><text x="260" y={y+11} class="small">{branch.length_cm} cm</text>
  {/each}
 {:else if diagram.kind==='sliding'}
  <line x1="40" y1="92" x2="220" y2="92" class="main-line"/><circle cx="115" cy="92" r="7" class="junction"/><line x1="115" y1="99" x2="115" y2="119" class="main-line"/><rect x="91" y="119" width="48" height="24" rx="5" class="weight"/><text x="95" y="135" class="small">piombo</text>
  <circle cx="220" cy="92" r="6" class="junction"/><line x1="226" y1="92" x2="328" y2="125" class="branch-line"/><circle cx="328" cy="125" r="3" class="branch-fill"/>
  <text x="35" y="65" class="label">Passafilo scorrevole</text><text x="195" y="74" class="small">girella</text><text x="170" y="162" class="label">Terminale {diagram.terminal_cm} cm</text><text x="290" y="145" class="small">amo</text>
 {:else if diagram.kind==='float'}
  <line x1="90" y1="38" x2="90" y2="220" class="main-line"/><ellipse cx="90" cy="48" rx="17" ry="22" class="float"/><text x="120" y="54" class="label">Galleggiante {fmt(diagram.float_g || 0)} g</text>
  {#each weights as weight}
   {@const y=205-(weight.height_cm/maxHeight)*125}
   {#if weight.kind==='torpille'}<ellipse cx="90" cy={y} rx="5" ry="9" class="main-fill"/>{:else}<circle cx="90" cy={y} r="4" class="main-fill"/>{/if}<text x="106" y={y+4} class="small">{weight.kind==='torpille'?'Torpille':'Pallini'} · {fmt(weight.height_cm)} cm · {fmt(weight.mass_g)} g</text>
  {/each}
  <circle cx="90" cy="220" r="5" class="junction"/><line x1="95" y1="222" x2="260" y2="247" class="branch-line"/><circle cx="260" cy="247" r="2" class="branch-fill"/>
  <text x="105" y="271" class="small">Terminale {diagram.terminal_cm} cm · piombatura iniziale {fmt(total)} g</text>
 {/if}
</svg>
<figcaption>Schema non in scala. {diagram.kind==='float'?'Quote dei pesi sopra la giunzione del terminale.':diagram.kind==='bottom'?'Quote degli snodi dall’attacco del piombo.':'Controlla che la lenza scorra liberamente.'}</figcaption>
</figure>
