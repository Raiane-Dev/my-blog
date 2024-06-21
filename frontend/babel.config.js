module.exports = function (api) {
  api.cache(true);

  const presets = [
    '@babel/preset-env',
    '@babel/preset-react',
    '@babel/preset-typescript',
  ];

  const plugins = ['@babel/plugin-proposal-class-properties'];

  if (process.env.NODE_ENV === 'development') {
    plugins.push('react-refresh/babel');
  } else {
    plugins.push(['react-refresh/babel', { skipEnvCheck: true }]);
  }

  return {
    presets,
    plugins,
  };
};
