/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  eslint: {
    ignoreDuringBuilds: true
  },
  publicRuntimeConfig: {
    // remove private variables from processEnv
    processEnv: Object.fromEntries(Object.entries(process.env).filter(([key]) => key.includes('NEXT_PUBLIC_')))
  },
  output: 'standalone',
  experimental: {
    optimizePackageImports: ['@mantine/core', '@mantine/hooks'],
  },
};

module.exports = nextConfig;
