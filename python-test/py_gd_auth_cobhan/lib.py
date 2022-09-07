from dataclasses import dataclass
from cobhan.cobhan import Cobhan

@dataclass
class AuthConfig:
    host: str

    def to_dict(self):
        return {
            'host': self.host
        }

class CobhanDemoLib(Cobhan):
    CDEFINES = """
        int32_t parse(void *config, void *token, void *output);
    """

    @classmethod
    def from_library_path(cls, library_root_path):
        instance = cls()
        instance.load_library(library_root_path, 'libcobhandemo', CobhanDemoLib.CDEFINES)
        return instance

    @classmethod
    def from_library_file(cls, library_file_path):
        instance = cls()
        instance.load_library_direct(library_file_path, CobhanDemoLib.CDEFINES)
        return instance

    def parse(self, config: AuthConfig, token: str):
        config_buf = self.to_json_buf(config.to_dict())

        token_buf = self.str_to_buf(token)

        output_len = int(len(token_buf) * 1.5) # Allow extra space for reformatting
        output_buf = self.allocate_buf(output_len)

        result = self._lib.parse(config_buf, token_buf, output_buf)
        if result < 0:
            raise Exception(f"parse failed {result}")

        return self.from_json_buf(output_buf)
