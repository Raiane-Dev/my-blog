module.exports = function (api) {
    api.cache(true);
  
    const presets = [
      '@babel/preset-env',
      '@babel/preset-react',
      '@babel/preset-typescript',
    ];
  
    const plugins = ['@babel/plugin-proposal-class-properties'];
  
    if (process.env.NODE_ENV === 'development') {
      // Ative o React Refresh apenas em ambiente de desenvolvimento
      plugins.push('react-refresh/babel');
    } else {
      // Ignore a verificação de ambiente em produção
      plugins.push(['react-refresh/babel', { skipEnvCheck: true }]);
    }
  
    return {
      presets,
      plugins,
    };
  };
  