import { globalStyle } from "@vanilla-extract/css"
import { theme } from "./theme"


globalStyle('*', {
  boxSizing: 'border-box',
  margin: 0,
  padding: 0,
})

globalStyle(':root', {
  backgroundColor: theme.dynamicTheme.colors.background,
})

globalStyle('html', {
  overflow: 'hidden',
  height: '100%',
})

globalStyle('main', {
  minHeight: '100vh',
  width: '100vw',
})

globalStyle('body', {
  height: '100%',
  overflow: 'auto',
})