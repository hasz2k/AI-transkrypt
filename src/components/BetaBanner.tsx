import { useLanguage } from "../LanguageContext";

export function BetaBanner() {
    const { t } = useLanguage();
    return (
        <div className="beta-banner">
        <strong>{t("beta")}</strong>
        <span>
        {t("betaMessage")}{" "}
        <a
        href="https://github.com/hasz2k/AI-transkrypt/issues"
        target="_blank"
        rel="noreferrer"
        style={{ color: "var(--accent)" }}
        >
        GitHub
        </a>
        </span>
        </div>
    );
}
