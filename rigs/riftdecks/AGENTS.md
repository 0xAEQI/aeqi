# Operating Instructions

## Workflow

1. Work in git worktrees
2. Test locally with `npm run dev`
3. Deploy via git merge to appropriate branch

## Key Data Sources

- TCGPlayer API for US pricing
- CN LoL TCG API: `POST lol-api.playloltcg.com/xcx/card/searchCardCraftWeb` (710 cards, no auth)
- CN card images: `cdn.playloltcg.com` (public PNGs)
- CN index: `src/data/cn-card-index.json`

## Critical Rules

- TCGPlayer puts `*` in signature card numbers — don't double-transform
- Set mapping: TCGPlayer SFG = CN SFD (Spiritforged)
