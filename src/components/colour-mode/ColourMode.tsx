import {
  createContext, ReactNode, useContext, useEffect,
  useState
} from 'react'
import { darkTheme, lightTheme } from '../../styles'
import { Box } from '../system/box/Box'
import * as styles from './ColourMode.css'

type ColorMode = typeof darkTheme | typeof lightTheme
interface ColorModeContextValues {
  colorMode: ColorMode | null
  setColorMode: (colorMode: ColorMode) => void
}

export const ColorModeContext = createContext<ColorModeContextValues>({
  colorMode: null,
  setColorMode: () => { },
})

export function ColorModeProvider({ children }: { children: ReactNode }) {
  const [colorMode, setColorMode] = useState<ColorMode | null>(null)

  useEffect(() => {
    setColorMode(
      document.documentElement.classList.contains(darkTheme) ? darkTheme : lightTheme,
    )
  }, [])

  const setter = (c: ColorMode) => {
    setColorMode(c)

    document.documentElement.classList.remove(lightTheme, darkTheme)
    document.documentElement.classList.add(c)
  }

  return (
    <ColorModeContext.Provider
      value={{
        colorMode,
        setColorMode: setter,
      }}
    >
      {children}
    </ColorModeContext.Provider>
  )
}

export const ColorModeToggle = () => {
  const { colorMode, setColorMode } = useContext(ColorModeContext)

  return (
    <Box
      component="button"
      cursor="pointer"
      className={styles.root}
      title="Toggle colour mode"
      onClick={() => setColorMode(colorMode === lightTheme ? darkTheme : lightTheme)}
    />
  )
}
