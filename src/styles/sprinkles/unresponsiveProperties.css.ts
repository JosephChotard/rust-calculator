import { defineProperties } from "@vanilla-extract/sprinkles"
import { theme } from "../theme"

export const unresponsiveProperties = defineProperties({
  properties: {
    flexWrap: ['wrap', 'nowrap'],
    top: [0],
    bottom: [0],
    left: [0],
    right: [0],
    flexShrink: [0],
    flexGrow: [0, 1],
    zIndex: [-1, 0, 1],
    width: { full: '100%' },
    borderRadius: theme.border.radius,
    cursor: ['pointer'],
  },
})