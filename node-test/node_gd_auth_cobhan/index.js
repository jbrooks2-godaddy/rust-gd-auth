import cobhan from 'cobhan'

const libgdauthcobhan = cobhan.load_platform_library('node_modules/node_gd_auth_cobhan/binaries', 'librust_gd_auth', {
    'parse': ['int32', ['pointer', 'pointer', 'pointer']],
    });

/**
* @param {object} config
* @param {string} token
* @return {object}
*/
function parse(config, token) {
    const config_json = JSON.stringify(config);
    const configBuffer = cobhan.string_to_cbuffer(config_json);
    const tokenBuffer = cobhan.string_to_cbuffer(token);
    const outputBuffer = cobhan.allocate_cbuffer(token.length);

    const result = libgdauthcobhan.parse(configBuffer, tokenBuffer, outputBuffer);
    if (result < 0) {
        throw new Error('parse failed: ' + result);
    }

    return JSON.parse(cobhan.cbuffer_to_string(outputBuffer));
}

export default { parse };