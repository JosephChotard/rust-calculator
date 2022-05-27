import { StyleRule } from "@vanilla-extract/css"
import { Properties, SimplePseudos } from "csstype"

export const breakpoints = {
  mobile: 0,
  tablet: 768,
  desktop: 1200,
}

export type Breakpoint = keyof typeof breakpoints

type CSSProps = Properties<string | number> & {
  [P in SimplePseudos]?: Properties<string | number>
}

export const queries = Object.fromEntries(
  Object.entries(breakpoints).filter(([key]) => key !== 'mobile').map(([key, bp]) => [
    key, `screen and (min-width: ${bp}px)`
  ])
)

const makeMediaQuery =
  (breakpoint: keyof typeof queries) => (styles: Properties<string | number>) =>
    !styles || Object.keys(styles).length === 0
      ? {}
      : {
        [queries[breakpoint]]: styles,
      }

const mediaQuery = {
  tablet: makeMediaQuery('tablet'),
  desktop: makeMediaQuery('desktop'),
}

interface ResponsiveStyle {
  mobile?: CSSProps
  tablet?: CSSProps
  desktop?: CSSProps
}

export const responsiveStyle = ({
  mobile,
  tablet,
  desktop,
}: ResponsiveStyle): StyleRule => {

  const tabletStyles = !tablet || tablet === mobile ? null : tablet

  const stylesBelowDesktop = tabletStyles || mobile
  const desktopStyles =
    !desktop || desktop === stylesBelowDesktop ? null : desktop

  const hasMediaQueries = tabletStyles || desktopStyles

  return {
    ...mobile,
    ...(hasMediaQueries
      ? {
        '@media': {
          ...(tabletStyles ? mediaQuery.tablet(tabletStyles) : {}),
          ...(desktopStyles ? mediaQuery.desktop(desktopStyles) : {}),
        },
      }
      : {}),
  }
}

export const mapToProperty =
  <Property extends keyof Properties<string | number>>(
    property: Property,
    breakpoint?: Breakpoint,
  ) =>
    (value: string | number) => {
      const styleRule = { [property]: value }

      return breakpoint
        ? responsiveStyle({ [breakpoint]: styleRule })
        : styleRule
    }
