"use client";
import React from "react";
import { WavyBackground } from "../components/ui/wavy-background";

const Home = () => {
    return (
        <div className="flex flex-col min-h-screen bg-black text-white">
            {/* Hero Section */}
            <WavyBackground className="max-w-4xl mx-auto pb-40">
                <h1 className="text-3xl md:text-5xl lg:text-7xl font-bold text-center mb-4">
                    Invest in real estate with tokens
                </h1>
                <p className="text-lg md:text-xl text-center mb-8">
                    Unlock fractional ownership, governance, and compliance in real estate investing.
                </p>
                <div className="flex justify-center gap-4 mb-8">
                    <button className="bg-blue-500 hover:bg-blue-600 text-white font-semibold px-6 py-3 rounded-full transition">
                        Browse Properties
                    </button>
                    <button className="bg-gray-800 hover:bg-gray-700 text-white font-semibold px-6 py-3 rounded-full transition border border-gray-600">
                        Connect Wallet
                    </button>
                </div>
            </WavyBackground>

            {/* How it Works Section */}
            <section className="max-w-3xl mx-auto py-12 px-4">
                <h2 className="text-2xl font-bold mb-4 text-center">How it works</h2>
                <ol className="list-decimal list-inside space-y-2 text-lg text-gray-200">
                    <li>Sign up and connect your crypto wallet.</li>
                    <li>Browse and invest in tokenized real estate properties.</li>
                    <li>Own fractional shares and participate in governance.</li>
                    <li>Receive rental income and track your portfolio.</li>
                </ol>
            </section>

            {/* Features Section */}
            <section className="max-w-4xl mx-auto py-12 px-4 grid md:grid-cols-3 gap-8">
                <div className="bg-gray-900 rounded-xl p-6 text-center shadow-lg">
                    <h3 className="text-xl font-semibold mb-2">Tokenization</h3>
                    <p>Invest with as little as $100. Own fractions of high-value properties via blockchain tokens.</p>
                </div>
                <div className="bg-gray-900 rounded-xl p-6 text-center shadow-lg">
                    <h3 className="text-xl font-semibold mb-2">Governance</h3>
                    <p>Vote on key property decisions and platform upgrades using your tokens.</p>
                </div>
                <div className="bg-gray-900 rounded-xl p-6 text-center shadow-lg">
                    <h3 className="text-xl font-semibold mb-2">Compliance</h3>
                    <p>Fully compliant with KYC/AML regulations for secure and legal investing.</p>
                </div>
            </section>

            {/* Footer */}
            <footer className="mt-auto py-6 bg-gray-950 text-gray-400 text-center text-sm">
                <div className="flex justify-center gap-6 mb-2">
                    <a href="https://docs.example.com" target="_blank" rel="noopener noreferrer" className="hover:text-white underline">Docs</a>
                    <a href="https://github.com/example/real-estate-app" target="_blank" rel="noopener noreferrer" className="hover:text-white underline">GitHub</a>
                    <a href="mailto:contact@example.com" className="hover:text-white underline">Contact</a>
                </div>
                <div>&copy; {new Date().getFullYear()} Real Estate Tokenization Platform</div>
            </footer>
        </div>
    );
};

export default Home;
