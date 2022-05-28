import { calc } from '@vanilla-extract/css-utils'
import { defineProperties } from "@vanilla-extract/sprinkles"
import { breakpoints, theme } from "../theme"

const space = theme.spacing

const negativeSpace = {
  ['-xsmall']: `${calc(space.xsmall).negate()}`,
  ['-small']: `${calc(space.small).negate()}`,
  ['-medium']: `${calc(space.medium).negate()}`,
  ['-large']: `${calc(space.large).negate()}`,
  ['-xlarge']: `${calc(space.xlarge).negate()}`,
  ['-xxlarge']: `${calc(space.xxlarge).negate()}`,
  ['-xxxlarge']: `${calc(space.xxxlarge).negate()}`,
}

const margins = {
  ...space,
  ...negativeSpace,
}

export const responsiveProperties = defineProperties({
  conditions: Object.fromEntries(
    Object.entries(breakpoints).map(([key, bp]) => [
      key, key === 'mobile' ? {} : { '@media': `screen and (min-width: ${bp}px)` }
    ])
  ),
  defaultCondition: 'mobile',
  properties: {
    position: ['absolute', 'relative', 'fixed'],
    display: ['none', 'block', 'inline', 'inline-block', 'flex'],
    alignItems: ['flex-start', 'center', 'flex-end'],
    justifyContent: ['flex-start', 'center', 'flex-end', 'space-between'],
    flexDirection: ['row', 'row-reverse', 'column', 'column-reverse'],
    paddingTop: space,
    paddingBottom: space,
    paddingLeft: space,
    paddingRight: space,
    top: space,
    bottom: space,
    left: space,
    right: space,
    marginTop: margins,
    marginBottom: margins,
    marginLeft: margins,
    marginRight: margins,
    pointerEvents: ['none', 'auto'],
    opacity: [0, 1],
    textAlign: ['left', 'center'],
    maxWidth: theme.contentWidth,
  },
  shorthands: {
    padding: ['paddingTop', 'paddingBottom', 'paddingLeft', 'paddingRight'],
    paddingX: ['paddingLeft', 'paddingRight'],
    paddingY: ['paddingTop', 'paddingBottom'],
    margin: ['marginTop', 'marginBottom', 'marginLeft', 'marginRight'],
    marginX: ['marginLeft', 'marginRight'],
    marginY: ['marginTop', 'marginBottom'],
  },
})

