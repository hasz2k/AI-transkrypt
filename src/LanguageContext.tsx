import { createContext, useContext, useState, ReactNode } from "react";
import { Lang, translations } from "./i18n";

type ContextType = {
  lang: Lang;
  setLang: (lang: Lang) => void;
  t: (key: keyof typeof translations.pl) => string;
};

const LanguageContext = createContext<ContextType | null>(null);

export function LanguageProvider({ children }: { children: ReactNode }) {
  const [lang, setLang] = useState<Lang>("pl");

  const t = (key: keyof typeof translations.pl) => translations[lang][key];

  return (
    <LanguageContext.Provider value={{ lang, setLang, t }}>
      {children}
    </LanguageContext.Provider>
  );
}

export function useLanguage() {
  const ctx = useContext(LanguageContext);
  if (!ctx) throw new Error("useLanguage must be used within LanguageProvider");
  return ctx;
}