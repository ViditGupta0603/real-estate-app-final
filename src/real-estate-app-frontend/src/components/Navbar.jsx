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

// Custom NavbarLogo to use local logo
const CustomNavbarLogo = () => (
    <Link
        to="/"
        className="relative z-20 mr-4 flex items-center space-x-2 px-2 py-1 text-sm font-normal text-black"
    >
        <img src="/logo.png" alt="logo" width={30} />
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

    return (
        <div className="relative w-full">
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
                    <div className="flex items-center gap-4">
                        <NavbarButton variant="primary">Connect Wallet</NavbarButton>
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
                                onClick={() => setIsMobileMenuOpen(false)}
                                variant="primary"
                                className="w-full">
                                Connect Wallet
                            </NavbarButton>
                        </div>
                    </MobileNavMenu>
                </MobileNav>
            </Navbar>
        </div>
    );
}

export default NavbarDemo;

