# Getting Started

```sh
sh scripts/usage.sh
```



# Cube

### Schema
```json
{
  "type": "object",
  "required": ["albedo", "position", "size", "scale", "type"],
  "properties": {
    "albedo": {
      "$ref": "#/definitions/LDRColor"
    },
    "position": {
      "$ref": "#/definitions/Position"
    },
    "size": {
      "$ref": "#/definitions/Size"
    },
    "scale": {
      "$ref": "#/definitions/Scale"
    },
    "type": {
      "type": "string",
      "enum": ["cube"]
    }
  }
}
```
