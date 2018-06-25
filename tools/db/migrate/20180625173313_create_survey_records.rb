class CreateSurveyRecords < ActiveRecord::Migration[5.2]
  def change
    create_table :survey_records do |t|
      t.references :field, null: false
      t.string :order, null: false, limit: 36
      t.text :value
      t.timestamps
    end
    add_index :survey_records, :order
    add_index :survey_records, %i[field_id order], unique: true
  end
end
