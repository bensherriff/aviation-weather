/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  eslint: {
    ignoreDuringBuilds: true
  },
  // webpackDevMiddleware: (config) => {
  //   config.watchOptions = {
  //     poll: 1000,
  //     aggregateTimeout: 300
  //   };
  //   return config;
  // },
  publicRuntimeConfig: {
    // remove private variables from processEnv
    processEnv: Object.fromEntries(Object.entries(process.env).filter(([key]) => key.includes('NEXT_PUBLIC_')))
  },
  output: 'standalone'
};

module.exports = nextConfig;
