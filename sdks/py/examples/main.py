import base64, rasn

# @rasn.sequence
class UserRecord:
    id: rasn.types.Integer
    oid: rasn.types.ObjectIdentifier
    # age: rasn.types.Integer = Field(tag=(TagKind.IMPLICIT, 0))

# @rasn.sequence(tag=(Tag.EXPLICIT, 1))
# class Department:
#     name: rasn.types.OctetString
#     code: rasn.types.Integer
#
# user = UserRecord(id=123, data=b"test", age=30)
# encoded = user.encode(rasn.Codec.DER)
#
# print(base64.b64encode(encoded))
# print(UserRecord.decode(encoded))
