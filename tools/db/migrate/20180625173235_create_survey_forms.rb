class CreateSurveyForms < ActiveRecord::Migration[5.2]
  def change
    create_table :survey_forms do |t|
      t.references :user, null: false
      t.string :title, null: false, limit: 255
      t.text :body, null: false
      t.string :media_type, null: false, limit: 8
      t.string :uid, null: false, limit: 36
      t.string :mode, null: false, limit: 16
      t.date :nbf, null: false
      t.date :exp, null: false
      t.timestamps
    end
    add_index :survey_forms, :title
    add_index :survey_forms, :uid, unique: true
  end
end
