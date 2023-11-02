module.exports = {
  mode: 'jit',
  content: {
    files: ['src/**/*.rs', 'index.html'],
  },
  darkMode: 'media',
  theme: {
    extend: {
      backgroundImage: {
        'banner-woman': "url('img/woman.png')",
      },
      fontFamily: { sans: ['Nunito Sans', 'sans'] },
      fontSize: {
        11: '11px',
        12: '12px',
        14: '14px',
        24: '24px',
        16: '16px',
      },
      lineHeight: {
        15: '15px',
        16: '16px',
        20: '20px',
        32: '32px',
        21: '21px',
      },
      fontWeight: {
        300: 300,
        400: 400,
        600: 600,
        700: 700,
      },
      colors: {
        primary: '#C53A3A',
        success: '#83BF94',
        pending: '#E17E23',
        info: '#9177DB',
        warning: '#DA2D38',
        grey: {
          'shade-0': '#232323',
          'shade-1': '#343434',
          'shade-2': '#464646',
          'shade-3': '#5B5B5B',
          'shade-4': '#6F6F6F',
          'shade-5': '#808080',
          'shade-6': '#949494',
          'shade-7': '#AAAAAA',
          'shade-8': '#BCBCBC',
          'shade-9': '#D1D1D1',
          'shade-10': '#E2E2E2',
          'shade-11': '#ECECEC',
          'shade-12': '#F0F0F0',
          'shade-13': '#F6F6F6',
          'shade-14': '#FFFFFF',
        },
      },
    },
    variants: {
      extend: {
        backgroundColor: ['hover', 'focus', 'active', 'disabled'],
        textColor: ['hover', 'focus', 'active', 'disabled'],
        borderColor: ['hover', 'focus', 'active', 'disabled'],
      },
    },
  },
  plugins: [],
};
