# This lib will build the extend type for daily development

1. Extend the basic type
2. Make validation the default part of data-type
3. Make serialize and store to Database smoothly

## 1. extend the basic type
1. Big Integer
2. Big Decimal
3. DateTime
4. ZonedDataTime
5. Birthday
6. Email
7. Cellphone
8. LongId
9. Uuid
10. JsonRecord

## 2. Make validation default

## 3. Serialize and deserialize 
### 3.1 From/To Serde
### 3.2 From/To Sqlx





## Global Structure
trait Contraint -> struct {Xxx}Constraint-> enum SupportedConstraint -> struct NamedConstraint
trait Field     -> struct {Xxx}Field     -> enum FieldType  -> struct ValidField       -> struct NamedField
Constraint array -> Schema
Field array -> Record

## trait Constraint
fn is_option(&self) -> bool
fn is_valid(&self, value) -> bool
fn is_valid_json(&self, json) -> bool
fn validate(&self, value) -> Result<(), ConstraintError>
fn validate_json(&self, value) -> Result<(), ConstraintError>

## trait Field
validate
From/To Serde
From/To Sqlx
Deref
DerefMut

## struct Schema(Constraint Array)
fn validate(&self, record) -> Result<(), ConstraintError>
fn is_valid(&self, record) -> bool
fn get_entity_schema(&self)  
fn get_selected_schema(&self)
fn get_primary_schema(&self)
fn get_location_schema(&self)
fn get_mutation_schema(&self)

## struct Record(Fields Array)
struct EntityRecord   : Entity
struct SelectedRecord : SelectedEntity
struct PrimaryRecord  : Primary
struct LocationRecord : Location
struct MutationRecord : Mutation
fn into_entity_record(&self, entity_schema) -> Result<Record, ConstraintError>
fn into_selected_record(&self, selected_schema) -> Result<Record, ConstraintError>
fn into_primary_record(&self, primary_schema) -> Result<Record, ConstraintError>
fn into_location_record(&self, location_schema) -> Result<Record, ConstraintError>
fn into_mutation_record(&self, mutation_schema) -> Result<Record, ConstraintError>

