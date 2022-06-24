import { tauri } from '@tauri-apps/api'
import {
  createContext, FC, ReactNode, useContext, useEffect,
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
    tauri.invoke<boolean>('get_system_theme').then((dark_mode) => {
      setter(dark_mode ? darkTheme : lightTheme)
    })
  }, [])

  const requestChange = (colorMode: ColorMode) => {
    setter(colorMode)
    tauri.invoke<void>('set_system_theme', { darkMode: colorMode === darkTheme })
      .catch((err) => console.error(err))
  }

  const setter = (c: ColorMode) => {
    setColorMode(c)

    document.documentElement.classList.remove(lightTheme, darkTheme)
    document.documentElement.classList.add(c)
  }

  return (
    <ColorModeContext.Provider
      value={{
        colorMode,
        setColorMode: requestChange,
      }}
    >
      {children}
    </ColorModeContext.Provider>
  )
}

export const ColorModeToggle: FC = () => {
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
