"use client";
import {
    Navbar,
    NavBody,
    NavItems,
    MobileNav,
    NavbarLogo,
    NavbarButton,
    MobileNavHeader,
    MobileNavToggle,
    MobileNavMenu,
} from "./ui/resizable-navbar";

import { useState } from "react";
import { Link } from "react-router-dom";

// Helper to shorten principal
function shortenPrincipal(principal) {
    if (!principal) return "";
    return principal.slice(0, 8) + "..." + principal.slice(-5);
}

// Custom NavbarLogo to use local logo
const CustomNavbarLogo = () => (
    <Link
        to="/"
        className="relative z-20 mr-4 flex items-center space-x-2 px-2 py-1 text-sm font-normal text-black"
    >
        <div style={{
            background: "white",
            borderRadius: "30%",
            padding: "2px",
            display: "flex",
            alignItems: "center",
            justifyContent: "center"
        }}>
            <img src="/real-estate.svg" alt="logo" width={35} height={25} />
        </div>
        <span className="font-bold text-lg text-black dark:text-white">RealEstate</span>
    </Link>
);

const NavbarDemo = () => {
    const navItems = [
        {
            name: "Home",
            link: "/",
        },
        {
            name: "Properties",
            link: "/properties",
        },
        {
            name: "Portfolio",
            link: "/portfolio",
        },
    ];

    const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
    const [principal, setPrincipal] = useState(null);
    const [connecting, setConnecting] = useState(false);
    const [copied, setCopied] = useState(false);

    const handleCopy = async () => {
        if (principal) {
            try {
                await navigator.clipboard.writeText(principal);
                setCopied(true);
                setTimeout(() => setCopied(false), 1200);
            } catch (err) {
                // Fallback for older browsers or non-secure origins
                try {
                    const textarea = document.createElement('textarea');
                    textarea.value = principal;
                    document.body.appendChild(textarea);
                    textarea.select();
                    document.execCommand('copy');
                    document.body.removeChild(textarea);
                    setCopied(true);
                    setTimeout(() => setCopied(false), 1200);
                } catch (fallbackErr) {
                    console.error(fallbackErr);
                }
            }
        }
    };

    // Connect to Plug Wallet
    const connectWallet = async () => {
        if (!window.ic || !window.ic.plug) {
            alert("Plug Wallet not found. Please install the Plug extension.");
            return;
        }
        setConnecting(true);
        try {
            const connected = await window.ic.plug.requestConnect();
            if (connected) {
                const principalObj = await window.ic.plug.getPrincipal();
                let principalId = "";
                if (typeof principalObj === "string") {
                    principalId = principalObj;
                } else if (principalObj && typeof principalObj.toText === "function") {
                    principalId = principalObj.toText();
                }
                if (principalId) {
                    setPrincipal(principalId);
                } else {
                    alert("Failed to get principal from Plug Wallet.");
                    setPrincipal(null);
                }
            }
        } catch (e) {
            alert("Failed to connect to Plug Wallet");
            setPrincipal(null);
            console.error(e);
        }
        setConnecting(false);
    };

    return (
        <div className="relative w-full bg-neutral-900">
            <Navbar>
                {/* Desktop Navigation */}
                <NavBody>
                    <CustomNavbarLogo />
                    <div className="flex-1 flex items-center justify-center">
                        {navItems.map((item, idx) => (
                            <Link
                                key={item.name}
                                to={item.link}
                                className="px-4 py-2 text-neutral-600 dark:text-neutral-300 hover:text-blue-500 dark:hover:text-blue-400 font-medium"
                            >
                                {item.name}
                            </Link>
                        ))}
                    </div>
                    <div className="flex flex-col items-end gap-1">
                        <NavbarButton
                            variant="primary"
                            onClick={principal ? undefined : connectWallet}
                            disabled={connecting || principal}
                            className={connecting || principal ? "cursor-not-allowed" : ""}
                        >
                            {principal
                                ? "Connected"
                                : connecting
                                    ? "Connecting..."
                                    : "Connect Wallet"}
                        </NavbarButton>
                        {principal && (
                            <div className="flex items-center gap-2 mt-1">
                                <span className="bg-gray-800 text-xs text-white px-2 py-1 rounded font-mono border border-gray-700">
                                    {shortenPrincipal(principal)}
                                </span>
                                <button
                                    onClick={handleCopy}
                                    className="text-xs text-blue-400 hover:text-blue-200 focus:outline-none"
                                    title="Copy to clipboard"
                                >
                                    {copied ? "Copied!" : "Copy"}
                                </button>
                            </div>
                        )}
                    </div>
                </NavBody>

                {/* Mobile Navigation */}
                <MobileNav>
                    <MobileNavHeader>
                        <CustomNavbarLogo />
                        <MobileNavToggle
                            isOpen={isMobileMenuOpen}
                            onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)} />
                    </MobileNavHeader>

                    <MobileNavMenu isOpen={isMobileMenuOpen} onClose={() => setIsMobileMenuOpen(false)}>
                        {navItems.map((item, idx) => (
                            <Link
                                key={`mobile-link-${idx}`}
                                to={item.link}
                                onClick={() => setIsMobileMenuOpen(false)}
                                className="block px-4 py-2 text-neutral-600 dark:text-neutral-300 hover:text-blue-500 dark:hover:text-blue-400 font-medium"
                            >
                                {item.name}
                            </Link>
                        ))}
                        <div className="flex w-full flex-col gap-4 mt-4">
                            <NavbarButton
                                onClick={() => {
                                    if (!principal) connectWallet();
                                    setIsMobileMenuOpen(false);
                                }}
                                variant="primary"
                                className={`w-full ${connecting || principal ? "cursor-not-allowed" : ""}`}
                                disabled={connecting || principal}
                            >
                                {principal
                                    ? "Connected"
                                    : connecting
                                        ? "Connecting..."
                                        : "Connect Wallet"}
                            </NavbarButton>
                            {principal && (
                                <div className="flex items-center gap-2 mt-1">
                                    <span className="bg-gray-800 text-xs text-white px-2 py-1 rounded font-mono border border-gray-700">
                                        {shortenPrincipal(principal)}
                                    </span>
                                    <button
                                        onClick={handleCopy}
                                        className="text-xs text-blue-400 hover:text-blue-200 focus:outline-none"
                                        title="Copy to clipboard"
                                    >
                                        {copied ? "Copied!" : "Copy"}
                                    </button>
                                </div>
                            )}
                        </div>
                    </MobileNavMenu>
                </MobileNav>
            </Navbar>
        </div>
    );
}

export default NavbarDemo;

