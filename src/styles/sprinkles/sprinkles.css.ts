import { createSprinkles } from "@vanilla-extract/sprinkles"
import { colorProperties } from "./colorProperties.css"
import { responsiveProperties } from "./responsiveProperties.css"
import { unresponsiveProperties } from "./unresponsiveProperties.css"

export const sprinkles = createSprinkles(
  responsiveProperties,
  unresponsiveProperties,
  colorProperties
)

export type Sprinkles = Parameters<typeof sprinkles>[0]