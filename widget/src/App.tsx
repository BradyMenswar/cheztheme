import { useCallback, useEffect, useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api/core";
import { BaseDirectory, watchImmediate } from "@tauri-apps/plugin-fs";
import { MagnifyingGlassIcon } from "@heroicons/react/16/solid";
import { InputGroup, Input } from "./components/input";
import { Button } from "./components/button";
import { Text } from "./components/text.tsx";
import { Badge } from "./components/badge";

export default function App() {
  const [config, setConfig] = useState<Config | null>(null);
  const [themes, setThemes] = useState<Theme[]>([]);
  const [currentThemeName, setCurrentThemeName] = useState<string>("");
  const [searchTerm, setSearchTerm] = useState<string>("");
  // const [error, setError] = useState<any>(null);

  const applyTheme = (selectedTheme: string) => {
    invoke("apply_theme", { themeName: selectedTheme });
  };

  const fetchConfig = useCallback(async () => {
    try {
      const result = await invoke<Config>("read_config");
      setConfig(result);
      // setError(null);
    } catch (err) {
      console.error("Error fetching config:", err);
      // setError(err);
      setConfig(null);
    }
  }, []);

  useEffect(() => {
    if (config && config.theme) {
      const root = document.documentElement;
      console.log("Applying new theme colors...");
			console.log(config.themeName)
	  setCurrentThemeName(config.themeName);
      for (const [key, hexValue] of Object.entries(config.theme)) {
        const rgbValue = hexToRgb(hexValue);
        if (rgbValue) {
          root.style.setProperty(`--color-${key}`, rgbValue);
        }
      }
    }
  }, [config]);

  useEffect(() => {
    fetchConfig();
    watchImmediate(
      ".config/chezmoi/chezmoi.toml",
      () => {
        fetchConfig();
      },
      {
        baseDir: BaseDirectory.Home,
      },
    );
  }, [fetchConfig]);

  useEffect(() => {
    const fetchThemes = async () => {
      try {
        const result = await invoke<Theme[]>("get_theme_names");
        setThemes(result);
      } catch (error) {
        console.error(error);
      }
    };
    fetchThemes();
  }, []);

  const filteredThemes = themes.filter((theme) =>
    theme.name.toLowerCase().includes(searchTerm.toLowerCase()),
  );

  return (
    <div className="flex h-screen overflow-hidden flex-col bg-custom-base00">
      <div className="p-4 bg-custom-base01/20">
        <InputGroup>
          <MagnifyingGlassIcon className="!text-custom-base05" />
          <Input
            name="search"
            placeholder="Search&hellip;"
            aria-label="Search"
            onChange={(e) => setSearchTerm(e.target.value)}
          />
        </InputGroup>
      </div>
      <div className="flex flex-col gap-3 p-4 overflow-y-auto">
        {filteredThemes.map((theme) => {
          return (
            <Button
              className="flex flex-col gap-2"
              color="custom"
              onClick={() => applyTheme(theme.name)}
            >
              <div className="flex gap-2 items-center">
                <Text className="!text-custom-base06">{theme.name}</Text>
                {theme.type === "preset" && <Badge color="base0A">Preset</Badge>}
                {theme.name === currentThemeName && <Badge color="base0B">Current</Badge>}
              </div>
              <div className="flex">
                {Object.values(theme.palette).map((hexValue) => {
                  return (
                    <div
                      className="size-4"
                      style={{ backgroundColor: hexValue }}
                    ></div>
                  );
                })}
              </div>
            </Button>
          );
        })}
      </div>
    </div>
  );
}

interface Config {
  themeName: string;
  theme: Palette;
}

export interface Palette {
  base00: string;
  base01: string;
  base02: string;
  base03: string;
  base04: string;
  base05: string;
  base06: string;
  base07: string;
  base08: string;
  base09: string;
  base0A: string;
  base0B: string;
  base0C: string;
  base0D: string;
  base0E: string;
  base0F: string;
}

interface Theme {
  id: string;
  name: string;
  type: string;
  palette: Palette;
}

function hexToRgb(hex: string) {
  if (!hex || hex.length < 4) return null;
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  if (!result) return null;
  const r = parseInt(result[1], 16);
  const g = parseInt(result[2], 16);
  const b = parseInt(result[3], 16);
  return `${r} ${g} ${b}`;
}
