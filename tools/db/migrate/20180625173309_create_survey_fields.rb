class CreateSurveyFields < ActiveRecord::Migration[5.2]
  def change
    create_table :survey_fields do |t|
      t.references :form, null: false
      t.string :name, null: false, limit: 32
      t.string :label, null: false, limit: 255
      t.text :options
      t.string :type, null: false, limit: 16
      t.boolean :required, null: false
      t.integer :sort, null: false, limit: 1
      t.timestamps
    end
    add_index :survey_fields, :label
    add_index :survey_fields, %i[form_id name], unique: true
  end
end
